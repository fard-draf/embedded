use chrono::{self, NaiveDate, NaiveTime};
//==================================================================================
#[derive(Debug, Default)]
pub struct DataBrooker {
    pub position: Position,
    pub cog: TrueCap,
    pub speed: Speed,
    pub time_stamp: TimeStamp,
    pub is_reliable: bool,
    pub altitude: Option<f32>,
    pub voltage: Option<f32>,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Speed(pub Option<f32>);

#[derive(Debug, Default)]
pub struct TrueCap(pub Option<f32>);

#[derive(Debug, Default)]
pub struct TimeStamp {
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
}

