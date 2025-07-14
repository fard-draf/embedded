#![no_std]

pub mod domain;
pub mod parsing;

pub const NMEA_MAX_LEN: usize = 100;
pub const NMEA_TRAM_COUNT: usize = 15;