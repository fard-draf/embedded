use crate::domain::GpsData;
use esp_hal::{
    uart::{self, Uart},
    Blocking,
};
use nb::block;
//==================================================================================
pub trait ByteSource {
    type Error;

    fn read_byte_blocking(&mut self) -> Result<u8, Self::Error>;
}

impl<'d> ByteSource for Uart<'d, Blocking> {
    type Error = uart::Error;

    fn read_byte_blocking(&mut self) -> Result<u8, Self::Error> {
        block!(self.read_byte())
    }
}
//==================================================================================
pub trait DataSink {
    type Error;

    fn process_data(&mut self, data: &GpsData) -> Result<(), Self::Error>;
}

pub struct ConsoleLogger;

#[derive(Debug)]
pub enum LoggerError {}

impl DataSink for ConsoleLogger {
    type Error = LoggerError;

    fn process_data(&mut self, data: &GpsData) -> Result<(), Self::Error> {
        esp_println::println!("{:#?}", data);
        Ok(())
    }
}
//==================================================================================
