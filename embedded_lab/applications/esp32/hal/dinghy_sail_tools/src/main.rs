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
//==================================================================================
use crate::display::display_mode_1;
use crate::display::display_mode_2;
use crate::display::display_init;
use crate::display::display_print;
use crate::domain::DataBrooker;
use crate::parsing::nmea_parsing_bytes;
use crate::ports::AdcByteSource;
use crate::battery::caclutate_battery_voltage;
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
//==================================================================================

#[entry]
fn main() -> ! {
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
                let display_setup = display_mode_2(&data_brooker);
                display_print(&mut display, &display_setup);
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
