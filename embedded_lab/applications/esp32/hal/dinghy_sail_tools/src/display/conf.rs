use crate::domain::DataBrooker;
use core::fmt::Write;
use heapless::String as HeaplessString;

pub fn data_print(
    gps_data: &DataBrooker,
) -> (
    HeaplessString<32>,
    HeaplessString<32>,
    HeaplessString<32>,
    HeaplessString<32>,
) {
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
