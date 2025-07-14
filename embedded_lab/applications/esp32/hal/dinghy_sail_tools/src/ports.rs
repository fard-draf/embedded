use crate::domain::GpsData;
use esp_hal::{
    analog::{adc::{Adc, AdcPin}}, uart::{self, Uart}, Blocking
};
use nb::block;

type AdcPinType = AdcPin<esp_hal::gpio::GpioPin<34>, esp_hal::peripherals::ADC1>;
//==================================================================================
pub trait GpsByteSource {
    type Error;

    fn read_byte_blocking(&mut self) -> Result<u8, Self::Error>;
}

impl<'a> GpsByteSource for Uart<'a, Blocking> {
    type Error = uart::Error;

    fn read_byte_blocking(&mut self) -> Result<u8, Self::Error> {
        block!(self.read_byte())
    }
}



//==================================================================================
pub trait AdcByteSource {
    type Error;

    fn read_value_blocking(&mut self, data: &mut AdcPinType) -> Result<u16, Self::Error>;
}

impl <'a> AdcByteSource for Adc<'a, esp_hal::peripherals::ADC1> {
    type Error = core::convert::Infallible;

    fn read_value_blocking(&mut self, data: &mut AdcPinType ) -> Result<u16, Self::Error> {
        let value = block!(self.read_oneshot(data)).unwrap();
        Ok(value)
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
