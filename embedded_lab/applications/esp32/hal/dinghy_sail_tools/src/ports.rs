use esp_hal::{
    analog::adc::{Adc, AdcPin},
};
use nb::block;

type AdcPinType = AdcPin<esp_hal::gpio::GpioPin<34>, esp_hal::peripherals::ADC1>;
//==================================================================================
pub trait AdcByteSource {
    type Error;

    fn read_value_blocking(&mut self, data: &mut AdcPinType) -> Result<u16, Self::Error>;
}

impl<'a> AdcByteSource for Adc<'a, esp_hal::peripherals::ADC1> {
    type Error = core::convert::Infallible;

    fn read_value_blocking(&mut self, data: &mut AdcPinType) -> Result<u16, Self::Error> {
        let value = block!(self.read_oneshot(data)).unwrap();
        Ok(value)
    }
}

//==================================================================================
