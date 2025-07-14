
pub const MAX_LAT_LEN: usize = 10;
pub const MAX_LONG_LEN: usize = 11;
pub const MAX_SPEED_LEN: usize = 5;
pub const MAX_TIME_LEN: usize = 6;
pub const MAX_DATE_UTC_LEN: usize = 6;
pub const NMEA_MAX_LEN: usize = 82;
pub const NMEA_TRAM_COUNT: usize = 2;


#[derive(Debug)]
pub struct GpsData {
    pub position: Position,
    pub speed: Speed,
    pub time_stamp: TimeStamp,
}

impl<'a> Default for GpsData {
    fn default() -> Self {
        Self {
            position: Position {
                latitude: [0u8; 11],
                longitude: [0u8; 12],
            },
            speed: Speed {
                speed_knot: 0.0,
                speed_kmh: 0.0,
            },
            time_stamp: TimeStamp {
                time: 000000,
                date: 000000,
            },
        }
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub latitude: [u8; 11],
    pub longitude: [u8; 12],
}

impl Position {
    pub fn new(latitude: &str, l_indic: &str, longitude: &str, g_indic: &str) -> Option<Self> {

        println!("latitude len {}", latitude.as_bytes().len()); // 10
        println!("latitude len {}", l_indic); // 1 

        println!("longitude len {}", longitude.as_bytes().len()); // 11
        println!("longitude len {}", g_indic); // 1 

        if latitude.len() == MAX_LAT_LEN && longitude.len() == MAX_LONG_LEN {

            let mut l_buffer = [0u8; MAX_LAT_LEN + 1 ]; // 10 + 1
            println!("lat_buffer {:?}", l_buffer.len());

            l_buffer[..latitude.len()].copy_from_slice(latitude.as_bytes());
            println!("lat_buffer {:?}", l_buffer);

            l_buffer[latitude.len() ..].copy_from_slice(l_indic.as_bytes());
            println!("lat_buffer {:?}", l_buffer);
            

            let mut g_buffer = [0u8; MAX_LONG_LEN + 1]; // 11 + 1
            println!("long_buffer {:?}", g_buffer.len());

            g_buffer[..longitude.len()].copy_from_slice(longitude.as_bytes());
            println!("long_buffer {:?}", g_buffer);

            g_buffer[longitude.len()..].copy_from_slice(g_indic.as_bytes());
            println!("long_buffer {:?}", g_buffer);

            Some(Self {
                latitude: l_buffer,
                longitude: g_buffer,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct Speed {
    pub speed_knot: f32,
    pub speed_kmh: f32,
}

impl Speed {
    pub fn new(sog: &str) -> Option<Self> {
        if sog.len() <= MAX_SPEED_LEN {
            let speed_knot = sog.parse::<f32>().unwrap_or_default();
            let speed_kmh = speed_knot * 1.852;

            Some(Self {
                speed_knot,
                speed_kmh,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct TimeStamp {
    pub time: u16,
    pub date: u16,
}

impl TimeStamp {
    pub fn new(time: &str, date_utc: &str) -> Option<Self> {
        if time.len() <= MAX_TIME_LEN && date_utc.len() <= MAX_DATE_UTC_LEN {
            let time = time.parse::<u16>().unwrap_or_default();
            let date = date_utc.parse::<u16>().unwrap_or_default();

            Some(Self { time, date })
        } else {
            None
        }
    }
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
pub struct TramSelected(pub [[u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT]);

impl Default for TramSelected {
    fn default() -> Self {
        Self([[0u8; NMEA_MAX_LEN]; NMEA_TRAM_COUNT])
    }
}



#[cfg(test)]
mod tests {


    use super::*;



    #[test]
    fn test_position() {
        let latitude = "4749.27884";
        let lat_indic = "N";
        let longitude = "00412.43573";
        let long_indic = "W";

        let position = Position {
            latitude: [52, 55, 52, 57, 46, 50, 55, 56, 56, 52, 78],
            longitude: [48, 48, 52, 49, 50, 46, 52, 51, 53, 55, 51, 87]
        };

        assert_eq!(Position::new(latitude, lat_indic, longitude, long_indic), Some(position));
    }

}

