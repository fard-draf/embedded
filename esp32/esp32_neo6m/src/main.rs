#![no_std]
#![no_main]
//==================================================================================
mod domain;
mod parsing;
mod ports;
mod utils;
mod conf;
//==================================================================================
use crate::domain::GpsData;
use crate::domain::TramConstructor;
use crate::parsing::process_received_byte;
use crate::ports::{ByteSource, ConsoleLogger, DataSink};
//==================================================================================
use esp_backtrace as _;
use esp_hal::{
    prelude::*,
    uart::{self, Uart},
};
use esp_println::println;

//==================================================================================
#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let config = uart::Config::default()
        .baudrate(9600)
        .parity_none()
        .stop_bits(uart::StopBits::STOP1);

    let uart = match Uart::new_with_config(
        peripherals.UART1,
        config,
        peripherals.GPIO16, //RX
        peripherals.GPIO17, //TX
    ) {
        Ok(uart_instance) => {
            println!("Uart OK");
            uart_instance
        }

        Err(e) => {
            println!("Uart default: {:?}", e);
            loop {}
        }
    };

    let mut byte_source: Uart<'_, esp_hal::Blocking> = uart;
    let mut data_sink = ConsoleLogger;

    let mut tram_constructor = TramConstructor::default();
    let mut gps_data = GpsData::default();

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
                        gps_data.sat_fix = gga.fix_satellites;
                    }

                    let _ = data_sink.process_data(&gps_data);

                    gps_data = GpsData::default();
                }
            }
            Err(e) => {
                esp_println::println!("\nError UART: {:#?}", e);
            }
        };
    }
}
