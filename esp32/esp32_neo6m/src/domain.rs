use crate::{NMEA_MAX_LEN, NMEA_TRAM_COUNT};

use chrono::{self, NaiveDate, NaiveTime};

const MAX_LAT_LEN: usize = 10;
const MAX_LONG_LEN: usize = 11;
const MAX_SPEED_LEN: usize = 5;
const MAX_TIME_LEN: usize = 6;
const MAX_DATE_UTC_LEN: usize = 6;

#[derive(Debug)]
pub struct GpsData {
    pub position: Position,
    pub cog: TrueCap,
    pub speed: Speed,
    pub time_stamp: TimeStamp,
    pub sat_fix: Option<u32>,
    pub altitude: Option<f32>,

}

// impl<'a> Default for GpsData {
//     fn default() -> Self {
//         Self {
//             position: Position {
//                 latitude: [0u8; 11],
//                 longitude: [0u8; 12],
//             },
//             speed: Speed {
//                 speed_knot: 0.0,
//                 speed_kmh: 0.0,
//             },
//             time_stamp: TimeStamp {
//                 time: 000000,
//                 date: 000000,
//             },
//         }
//     }
// }

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug)]
pub struct Speed (pub Option<f32>);



#[derive(Debug, Default)]
pub struct TrueCap (pub Option<f32>);

#[derive(Debug, Default)]
pub struct TimeStamp {
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
}



pub struct TramConstructor {
    pub rx_buffer: [u8; NMEA_MAX_LEN],
    pub rx_count: usize,
    pub frame_storage: [[u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT],
    pub frame_index: usize,
    pub parsed_gps_trams: [u8; NMEA_TRAM_COUNT],
}

impl Default for TramConstructor {
    fn default() -> Self {
        Self {
            rx_buffer: [0u8; NMEA_MAX_LEN],
            frame_storage: [[0u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT],
            parsed_gps_trams: [0; NMEA_TRAM_COUNT],
            rx_count: 0,
            frame_index: 0,
        }
    }
}

#[derive(Debug)]
pub struct TramSelected {
    pub rmc: [u8; NMEA_MAX_LEN],
    pub gga: [u8; NMEA_MAX_LEN],
}

impl Default for TramSelected {
    fn default() -> Self {
        Self {
            rmc: [0u8; NMEA_MAX_LEN],
            gga: [0u8; NMEA_MAX_LEN],
        }
    }
}
