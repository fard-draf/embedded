#![no_std]
#![no_main]
//==================================================================================
mod board;
mod display;
mod domain;
mod gps;
mod utils;
mod power_managment;
mod ports;
//==================================================================================
use crate::display::conf::data_print;
use crate::display::init::display_init;
use crate::display::init::display_print;
use crate::domain::GpsData;
use crate::domain::TramConstructor;
use crate::gps::parsing::nmea_parsing_bytes;
use crate::gps::parsing::process_received_byte;
use crate::ports::AdcByteSource;
use crate::ports::GpsByteSource;
use crate::power_managment::voltage::caclutate_batterie_voltage;
//==================================================================================

//==================================================================================
use esp_backtrace as _;
use esp_hal::peripherals;
use esp_hal::{prelude::*, uart::Uart, analog::adc::{Adc,AdcPin }};
use esp_println::println;
//==================================================================================
#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let drivers = board::init(peripherals);
    //==========================================
    let mut display = display_init(drivers.display_i2c);
    //==========================================
    let mut gps_byte_source: Uart<'_, esp_hal::Blocking> = drivers.gps_uart;
    let mut tram_constructor = TramConstructor::default();
    let mut gps_data = GpsData::default();
    //==========================================
    let mut adc1_byte_source = Adc::new(drivers.volt_adc.peripherals_adc1, drivers.volt_adc.adc1_conf );
    let mut adc1_pin: AdcPin<esp_hal::gpio::GpioPin<34>, peripherals::ADC1> = drivers.volt_adc.adc_pin;
    //==========================================
    loop {
        match adc1_byte_source.read_value_blocking(&mut adc1_pin) {
            Ok(raw_value) => {
                gps_data.voltage = caclutate_batterie_voltage(raw_value, &mut gps_data).voltage;
                
            }            
            Err(e) => {
                println!("Infaillible error: {:?}", e);
            }
        }

        match gps_byte_source.read_byte_blocking() {
            Ok(byte) => {
                if let Some(process) = process_received_byte(byte, &mut tram_constructor) {
                    nmea_parsing_bytes(&process, &mut gps_data);
                    let (date, time, speed, voltage) = data_print(&gps_data);
                    display_print(&mut display, date, time, speed, voltage);
                    gps_data = GpsData::default();
                }
            }
            Err(e) => {
                esp_println::println!("\nError UART: {:#?}", e);
            }
        };
    }
}
