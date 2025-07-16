#![no_std]

pub use chrono::{self, NaiveDate, NaiveTime};
pub use embedded_hal;
pub use nb;

use embedded_hal::serial::{Read, Write};
use nmea::ParseResult;

#[derive(Debug, Default)]
pub struct GpsData {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub speed_knots: Option<f32>,
    pub course: Option<f32>,
    pub variation: Option<f32>,
    pub altitude: Option<f32>,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub sat_count: Option<u32>,
    pub fix: Option<bool>,
}

pub struct Gps<SERIAL> {
    serial: SERIAL,
    data: GpsData,
    buffer: [u8; 100],
    buffer_len: usize,
}

impl<SERIAL> Gps<SERIAL>
where
    SERIAL: Read<u8> + Write<u8>,
{
    pub fn new(serial_port: SERIAL) -> Self {
        Self {
            serial: serial_port,
            data: GpsData::default(),
            buffer: [0u8; 100],
            buffer_len: 0,
        }
    }

    pub fn update(&mut self) -> Result<&GpsData, &str> {
        let mut new_data_parsed = false;

        loop {
            match self.serial.read() {
                Ok(byte) => {
                    if self.buffer_len < self.buffer.len() {
                        self.buffer[self.buffer_len] = byte;
                        self.buffer_len += 1;
                    }

                    // A newline indicates the end of a sentence
                    if byte == b'\n' {
                        if self.parse_and_translate() {
                            new_data_parsed = true;
                        }
                        // Reset buffer for the next sentence
                        self.buffer_len = 0;
                    }
                }
                Err(nb::Error::WouldBlock) => {
                    // No more data available right now. Exit the loop.
                    break;
                }
                Err(_) => {
                    self.buffer_len = 0; // Clear buffer on error
                    return Err("Serial read error");
                }
            }
        }

        if new_data_parsed {
            Ok(&self.data)
        } else {
            Err("No new data")
        }
    }
}

/// Private helper methods
impl<SERIAL> Gps<SERIAL> {
    /// Parses the internal buffer and translates NMEA data to `GpsData`.
    /// This is an internal helper function.
    fn parse_and_translate(&mut self) -> bool {
        match nmea::parse_bytes(&self.buffer[..self.buffer_len]) {
            Ok(parsed_sentence) => {
                // A sentence was successfully parsed, now translate it.
                self.translate_to_gps_data(parsed_sentence);
                true
            }
            Err(_e) => {
                // Failed to parse (invalid sentence, checksum error, etc.)
                // If the "log" feature is enabled, we log this.
                #[cfg(feature = "log")]
                log::warn!("NMEA parse error: {:?}", e);
                false
            }
        }
    }

    /// Translates the data from a `nmea::Sentence` to our `GpsData` struct.
    fn translate_to_gps_data(&mut self, message: ParseResult) {
        match message {
            ParseResult::RMC(data) => {
                self.data.latitude = data.lat;
                self.data.longitude = data.lon;
                self.data.speed_knots = data.speed_over_ground;
                self.data.course = data.true_course;
                self.data.time = data.fix_time;
                self.data.date = data.fix_date;
            }
            ParseResult::GGA(data) => {
                self.data.altitude = data.altitude;
                self.data.sat_count = data.fix_satellites;
                self.data.fix = {
                    if data.fix_satellites > Some(4) {
                        Some(true)
                    } else {
                        Some(false)
                    }
                };
            }
            _ => { /* Ignore other sentence types */ }
        }
    }
}
