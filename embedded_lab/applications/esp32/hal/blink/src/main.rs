#![no_std]
#![no_main]


use embedded_hal::digital::InputPin;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{self, Input, Io, Level, Output, Pull},
    i2c::master::{Config, I2c},
    prelude::*,
    analog::adc::{Adc,AdcChannel, AdcPin, AdcConfig},
};
use esp_println::{print, println};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    
    let mut led = Output::new(peripherals.GPIO2, Level::Low);




    let mut button = Input::new(peripherals.GPIO13, Pull::Up);


    let mut last_button_state = true;
    let mut mode = 1u8;

   loop {
        let button_state = button.is_high();

        
        if button_state != last_button_state && !button_state {
        }
        
        last_button_state = button_state;
        
        delay.delay_millis(10);
    }
}