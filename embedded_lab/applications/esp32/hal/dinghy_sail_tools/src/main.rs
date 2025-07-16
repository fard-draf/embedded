#![no_std]
#![no_main]
#![feature(const_mut_refs)]
//==================================================================================
mod board;
mod display;
mod domain;
mod ports;
mod parsing;
mod battery;
mod utils;
mod wasm;
//==================================================================================
use linked_list_allocator::LockedHeap;
use crate::display::display_mode_1;
use crate::display::display_mode_2;
use crate::display::display_init;
use crate::display::display_print;
use crate::domain::DataBrooker;
use crate::parsing::nmea_parsing_bytes;
use crate::ports::AdcByteSource;
use crate::battery::caclutate_battery_voltage;
use crate::wasm::wasm_init;
//==================================================================================
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
//==================================================================================
use gps_driver::Gps;
use esp_hal::delay::Delay;
use esp_backtrace as _;
use esp_hal::peripherals;
use esp_hal::{
    analog::adc::{Adc, AdcPin},
    prelude::*,
};
use esp_println::println;
use wasmi;
//==================================================================================

#[entry]
fn main() -> ! {
    //==========================================HEAP_ALLOC
    const HEAP_SIZE: usize = 200 * 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { ALLOCATOR.lock().init(HEAP.as_mut_ptr(), HEAP_SIZE )};
    //==========================================INIT_METAL
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let drivers = board::init(peripherals);
    let delay = Delay::new();
    //==========================================I2C_DISPLAY
    let mut display = display_init(drivers.display_i2c);
    //==========================================UART_GPS
    let mut data_brooker = DataBrooker::default();
    let mut gps = Gps::new(drivers.gps_uart);
    //==========================================ADC_VOLT
    let mut adc1_byte_source = Adc::new(
        drivers.volt_adc.peripherals_adc1,
        drivers.volt_adc.adc1_conf,
    );
    let mut adc1_pin: AdcPin<esp_hal::gpio::GpioPin<34>, peripherals::ADC1> =
        drivers.volt_adc.adc_pin;
    //==========================================WASM_DISPLAY
    let mut wasm_init = wasm_init();
    //==========================================LOOP
    loop {
        match adc1_byte_source.read_value_blocking(&mut adc1_pin) {
            Ok(raw_value) => {
                data_brooker.voltage =
                    caclutate_battery_voltage(raw_value, &mut data_brooker).voltage;
            }
            Err(e) => {
                println!("Infaillible error: {:?}", e);
            }
        }

        match gps.update() {
            Ok(gps_data) => {
                nmea_parsing_bytes(gps_data, &mut data_brooker);
                wasm_init.layout_func.call(
                    &mut wasm_init.store, 
                        (
                            data_brooker.speed.0.unwrap_or(0.0),
                            data_brooker.voltage.unwrap_or(0.0),
                            data_brooker.altitude.unwrap_or(0.0),
                        )
                ).unwrap();
                // let display_setup = display_mode_2(&data_brooker);
                // generate_layout_func.call(, params)
                display_print(&mut display, &wasm_init.store.data().layout);
                data_brooker = DataBrooker::default();
            }
            Err("No new data") => {
                delay.delay_millis(100);
            }
            Err(e) => {
                println!("Gps Error: {:?}", e)
            }
        }

    }
}
