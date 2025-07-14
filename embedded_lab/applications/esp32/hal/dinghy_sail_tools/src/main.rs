#![no_std]
#![no_main]
//==================================================================================
mod board;
mod display;
mod domain;
mod gps;
mod ports;
mod power_managment;
mod utils;
//==================================================================================
use crate::display::conf::data_print;
use crate::display::init::display_init;
use crate::display::init::display_print;
use crate::domain::DataBrooker;
use crate::gps::parsing::nmea_parsing_bytes;
use crate::ports::AdcByteSource;
use crate::power_managment::voltage::caclutate_batterie_voltage;
use esp_hal::delay::Delay;
use gps_driver::Gps;
//==================================================================================

//==================================================================================
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
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let drivers = board::init(peripherals);
    let delay = Delay::new();
    //==========================================
    let mut display = display_init(drivers.display_i2c);
    //==========================================
    let mut data_brooker = DataBrooker::default();
    let mut gps = Gps::new(drivers.gps_uart);
    //==========================================
    let mut adc1_byte_source = Adc::new(
        drivers.volt_adc.peripherals_adc1,
        drivers.volt_adc.adc1_conf,
    );
    let mut adc1_pin: AdcPin<esp_hal::gpio::GpioPin<34>, peripherals::ADC1> =
        drivers.volt_adc.adc_pin;
    //==========================================
    loop {
        match adc1_byte_source.read_value_blocking(&mut adc1_pin) {
            Ok(raw_value) => {
                data_brooker.voltage =
                    caclutate_batterie_voltage(raw_value, &mut data_brooker).voltage;
            }
            Err(e) => {
                println!("Infaillible error: {:?}", e);
            }
        }

        match gps.update() {
            Ok(gps_data) => {
                nmea_parsing_bytes(gps_data, &mut data_brooker);
                let (date, time, speed, voltage) = data_print(&data_brooker);
                display_print(&mut display, date, time, speed, voltage);
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
