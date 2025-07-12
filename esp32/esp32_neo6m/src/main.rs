#![no_std]
#![no_main]
//==================================================================================
mod board;
mod conf;
mod display;
mod domain;
mod parsing;
mod ports;
mod utils;
//==================================================================================
use crate::display::display_init;
use crate::display::display_print;
use crate::domain::GpsData;
use crate::domain::TramConstructor;
use crate::parsing::process_received_byte;
use crate::ports::ByteSource;
//==================================================================================
use esp_backtrace as _;
use esp_hal::{prelude::*, uart::Uart};

use core::fmt::Write;
use heapless::String as HeaplessString;
//==================================================================================
#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let drivers = board::init(peripherals);
    //==========================================
    let mut display = display_init(drivers.display_i2c);
    //==========================================
    let mut byte_source: Uart<'_, esp_hal::Blocking> = drivers.gps_uart;
    let mut tram_constructor = TramConstructor::default();
    let mut gps_data = GpsData::default();

    //==========================================
    loop {
        match byte_source.read_byte_blocking() {
            Ok(byte) => {
                if let Some(process) = process_received_byte(byte, &mut tram_constructor) {
                    if let Ok(nmea::ParseResult::RMC(rmc)) = nmea::parse_bytes(&process.rmc) {
                        gps_data.position.latitude = rmc.lat;
                        gps_data.position.longitude = rmc.lon;
                        gps_data.time_stamp.date = rmc.fix_date;
                        gps_data.time_stamp.time = rmc.fix_time;
                        gps_data.speed.0 = rmc.speed_over_ground;
                        gps_data.cog.0 = rmc.true_course;
                    }

                    if let Ok(nmea::ParseResult::GGA(gga)) = nmea::parse_bytes(&process.gga) {
                        gps_data.altitude = gga.altitude;
                        if gga.fix_satellites >= Some(6) {
                            gps_data.is_reliable = true
                        };
                    }

                    let mut date: HeaplessString<32> = HeaplessString::new();
                    if let Some(naive_date) = gps_data.time_stamp.date {
                        let _ = write!(date, "{:?}", naive_date);
                    }

                    let mut time: HeaplessString<32> = HeaplessString::new();
                    if let Some(naive_time) = gps_data.time_stamp.time {
                        let _ = write!(time, "{:?}", naive_time);
                    }

                    display_print(&mut display, date, time);

                    gps_data = GpsData::default();
                }
            }
            Err(e) => {
                esp_println::println!("\nError UART: {:#?}", e);
            }
        };
    }
}
