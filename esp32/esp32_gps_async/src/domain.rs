use crate::{NMEA_MAX_LEN, NMEA_TRAM_COUNT};

pub struct GpsData {
    position: Position,
    speed: Speed,
    time_stamp: TimeStamp,
}

pub struct Position {
    latitude: f64,
    longitude: f64,
}

pub struct Speed {
    speed_knot: f32,
    speed_kmh: f32,
}

pub struct TimeStamp {
    time: u16,
    date: u16,
}

pub struct TramConstructor {
    pub rx_buffer: [u8; NMEA_MAX_LEN],
    pub rx_count: usize,
    pub frame_storage: [[u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT],
    pub frame_index: usize,
    pub parsed_gps_trams: [u8; 15],
}

impl Default for TramConstructor {
    fn default() -> Self {
        Self{
        rx_buffer: [0u8; NMEA_MAX_LEN],
        frame_storage: [[0u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT],
        parsed_gps_trams: [0; NMEA_TRAM_COUNT],
        rx_count: 0,
        frame_index: 0,
        }
    }
}
