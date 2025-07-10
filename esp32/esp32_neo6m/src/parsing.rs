use crate::{domain::TramConstructor, NMEA_MAX_LEN, NMEA_TRAM_COUNT};
use esp_println::println;


pub fn process_received_byte(byte: u8, tram: &mut TramConstructor) {
            if byte as char == '$' {
                tram.rx_count = 0;
            }

            if tram.rx_count < NMEA_MAX_LEN {
                tram.rx_buffer[tram.rx_count] = byte;
                tram.rx_count += 1;
            }

            if byte as char == '\n' {
                let current_frame_slice = &tram.rx_buffer[0..tram.rx_count];

                if let Ok(frame_str) = core::str::from_utf8(current_frame_slice) {
                    if frame_str.starts_with("$GPRMC") && !tram.frame_storage.is_empty() {
                        for i in 0..tram.frame_index {
                            let length = tram.parsed_gps_trams[i];
                            let frame_slice = &tram.frame_storage[i][0..length as usize];
                            if let Ok(s) = core::str::from_utf8(frame_slice) {
                                println!("[{}] {}", i, s.trim_end());
                            }
                        }

                        tram.frame_index = 0;
                    }

                    if frame_str.starts_with("$GP") {
                        if tram.frame_index < NMEA_TRAM_COUNT {
                            tram.frame_storage[tram.frame_index][0..tram.rx_count]
                                .copy_from_slice(&tram.rx_buffer[0..tram.rx_count]);
                            tram.parsed_gps_trams[tram.frame_index] = tram.rx_count as u8;

                            println!(
                                "Signal stocked. Index: {} / Lenght: {}",
                                tram.frame_index, tram.rx_count
                            );
                            tram.frame_index += 1;
                        }
                    }
                }
                tram.rx_count = 0;
            }
}
       