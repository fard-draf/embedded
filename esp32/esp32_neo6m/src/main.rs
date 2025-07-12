#![no_std]
#![no_main]

mod domain;
mod parsing;
mod utils;

use crate::parsing::{parse_sentence, process_received_byte};
use crate::{domain::TramConstructor};

use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    uart::{self, Uart},
};
use esp_println::{print, println};
use nb::block;
use nmea::{Nmea, ParseResult};

pub const NMEA_MAX_LEN: usize = 82;
pub const NMEA_TRAM_COUNT: usize = 2;

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

    let mut nmea = Nmea::default();

    let mut tram_constructor = TramConstructor::default();

    loop {
        match block!(uart.read_byte()) {
            Ok(byte) => {
                if let Some(process) = process_received_byte(byte, &mut tram_constructor) {
                    
                    // if let Ok(nmea::ParseResult::RMC(rmc)) = nmea::parse_bytes(&process.0[0]) {
                    //     println!("[RMC OK]: Fix time {:?}, speed_over_ground {:?}", rmc.fix_time, rmc.speed_over_ground );
                    //     println!("[RMC OK]: Lat: {:?}, Long: {:?}, true course: {:?}", rmc.lat, rmc.lon, rmc.true_course);
                    //     println!("[RMC OK]: status_of_fix: {:?}, nav_status: {:?}, true faa_mode: {:?}", rmc.status_of_fix, rmc.nav_status, rmc.faa_mode);
                        
                    // }

                    // if let Ok(nmea::ParseResult::GGA(gga)) = nmea::parse_bytes(&process.0[1]) {
                    //     println!("[VTG OK]: fix_sat: {:?}, alt: {:?}", gga.fix_satellites, gga.altitude);
                    // }
                    
                    let res = parse_sentence(process);
                    println!("{:#?}", res);
                }
            }
            Err(e) => {
                esp_println::println!("\nError UART: {:#?}", e);
            }
        };
    }
}
