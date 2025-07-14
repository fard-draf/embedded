use core::fmt::Debug;

use heapless::mpmc::Q2;

use crate::{
    gps::conf::{NMEA_MAX_LEN, NMEA_TRAM_COUNT},
    domain::{GpsData, TramConstructor, TramSelected},
    ports::GpsByteSource,
};

//==================================================================================

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

//==================================================================================

pub fn nmea_parsing_bytes<'a>(process: &TramSelected, gps_data: &'a mut GpsData) -> &'a GpsData {

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
    gps_data
}
