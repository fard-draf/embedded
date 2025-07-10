#![no_std]
#![no_main]

mod domain;
mod parsing;

use crate::parsing::process_received_byte;
use crate::domain::TramConstructor;

use embedded_hal::digital::InputPin;
use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    uart::{self, Uart},
};
use esp_println::{print, println};
use nb::block;


pub const NMEA_MAX_LEN: usize = 100;
pub const NMEA_TRAM_COUNT: usize = 15;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let config = uart::Config::default()
        .baudrate(9600)
        .parity_none()
        .stop_bits(uart::StopBits::STOP1);

    let mut uart = Uart::new_with_config(
        peripherals.UART1,
        config,
        peripherals.GPIO16, //RX
        peripherals.GPIO17, //TX
    )
    .unwrap();

    let mut tram_constructor = TramConstructor::default();

    loop {
        match block!(uart.read_byte()) {
            Ok(byte) => { 
                process_received_byte(byte, &mut tram_constructor);

            }
            Err(e) => esp_println::println!("\nError UART: {:?}", e)
        }
    }
}


    

