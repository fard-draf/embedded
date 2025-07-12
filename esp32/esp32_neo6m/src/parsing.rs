use nmea::Nmea;

use crate::{
    domain::{GpsData, Position, Speed, TimeStamp, TramConstructor, TramSelected, TrueCap},
    NMEA_MAX_LEN, NMEA_TRAM_COUNT,
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
                // for i in 0..tram.frame_index {
                //     let length = tram.parsed_gps_trams[i];
                //     let frame_slice = &tram.frame_storage[i][0..length as usize];
                //     if let Ok(s) = core::str::from_utf8(frame_slice) {
                //         println!("[{}] {}", i, s.trim_end());
                //     }
                // }

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

pub fn parse_sentence(sentence: TramSelected) -> Option<GpsData> {
    if let Ok(nmea::ParseResult::GGA(gga)) = nmea::parse_bytes(&sentence.gga) {
        if gga.fix_satellites >= Some(4)  {

            if let Ok(nmea::ParseResult::RMC(rmc)) = nmea::parse_bytes(&sentence.rmc) {

                Some(GpsData {
                    position: Position {
                        latitude: rmc.lat,
                        longitude: rmc.lon
                    },
                    cog: TrueCap(rmc.true_course),
                    speed: Speed(rmc.speed_over_ground),
                    time_stamp: TimeStamp {
                        time: rmc.fix_time,
                        date: rmc.fix_date,
                    },
                    sat_fix: gga.fix_satellites,
                    altitude: gga.altitude
                })

            } else {None}
        } else {None}
    } else {None}
}