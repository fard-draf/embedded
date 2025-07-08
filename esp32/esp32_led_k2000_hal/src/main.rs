#![no_std]
#![no_main]


use embedded_hal::digital::InputPin;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{self, Input, Io, Level, Output, Pull},
    i2c::master::{Config, I2c},
    prelude::*,
};
use esp_println::{print, println};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    
    let mut leds = [
        Output::new(peripherals.GPIO2, Level::Low),
        Output::new(peripherals.GPIO4, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO19, Level::Low),
        
        ];


    for led in leds.iter_mut() {
        led.set_low();
    }

   loop {

        for i in  (0..leds.len()) {
            if i == 0 || i == leds.len() {
                leds[i].set_high();
                delay.delay_millis(75);
                leds[i].set_low();
        } else {
                leds[i].set_high();
                delay.delay_millis(150);
                leds[i].set_low();
        }
            }


        for i in (0..leds.len() - 1).rev(){
            if i == leds.len() || i == 0 {
                leds[i].set_high();
                delay.delay_millis(75);
                leds[i].set_low();
            } else {
                leds[i].set_high();
                delay.delay_millis(150);
                leds[i].set_low();
            }

        }
        
        }
        
    
    
}