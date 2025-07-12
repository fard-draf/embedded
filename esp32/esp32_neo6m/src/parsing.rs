use crate::{
    domain::{TramConstructor, TramSelected},
    conf::{NMEA_MAX_LEN,NMEA_TRAM_COUNT}
};

pub fn process_received_byte(byte: u8, tram: &mut TramConstructor) -> Option<TramSelected> {
    if byte as char == '$' {
    tram.rx_count = 0;
    }

    if tram.rx_count < NMEA_MAX_LEN {
        tram.rx_buffer[tram.rx_count] = byte;
        tram.rx_count += 1;
    }

    if byte as char == '\n' {
        let current_frame_slice = &tram.rx_buffer[0..tram.rx_count];
        let mut ret = TramSelected::default();

        if let Ok(frame_str) = core::str::from_utf8(current_frame_slice) {
            if frame_str.starts_with("$GPRMC") || frame_str.starts_with("$GPGGA") {
                
                if tram.frame_index < NMEA_TRAM_COUNT {
                    tram.frame_storage[tram.frame_index][0..tram.rx_count]
                        .copy_from_slice(&tram.rx_buffer[0..tram.rx_count]);
                    tram.parsed_gps_trams[tram.frame_index] = tram.rx_count as u8;
                    tram.frame_index += 1;

                }
            }

            if frame_str.starts_with("$GPGGA") && tram.frame_storage.len() > 1 {

                ret.rmc = tram.frame_storage[0];
                ret.gga = tram.frame_storage[1];
                tram.frame_index = 0;

                return Some(ret);
            }
        }
        tram.rx_count = 0;
    };
    None
}

