use crate::domain::GpsData;
use heapless::String as HeaplessString;
use core::fmt::Write;

pub fn data_print(gps_data: &GpsData) -> (HeaplessString<32>, HeaplessString<32>, HeaplessString<32>, HeaplessString<32>) {

    let mut date: HeaplessString<32> = HeaplessString::new();
    if let Some(naive_date) = gps_data.time_stamp.date {
        let _ = write!(date, "{:?}", naive_date);
    }

    let mut time: HeaplessString<32> = HeaplessString::new();
    if let Some(naive_time) = gps_data.time_stamp.time {
        let _ = write!(time, "{:?}", naive_time);
    }

    let mut speed: HeaplessString<32> = HeaplessString::new();
    if let Some(speed_val) = gps_data.speed.0 {
        let _ = write!(speed, "{:.2?} knots", speed_val);
    }
    
    let mut voltage: HeaplessString<32> = HeaplessString::new();
    if let Some(volt) = gps_data.voltage {
        let _ = write!(voltage, "{:.2?} volt", volt);
    }
    (date, time, speed, voltage)

}
