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

    
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    let mut button = Input::new(peripherals.GPIO13, Pull::Up);

    let (mut red, mut green, mut blue)= (Output::new(peripherals.GPIO21, Level::Low), Output::new(peripherals.GPIO22, Level::Low), Output::new(peripherals.GPIO23, Level::Low) );

    let mut last_button_state = true;
    let mut mode = 1u8;

   loop {
        let button_state = button.is_high();

        
        if button_state != last_button_state && !button_state {
            mode = if mode < 3 { mode + 1} else { 1 };
            println!("Button_state {}", button_state);
            match mode {
                1 => {
                    println!("Red");
                    println!("mode {}", mode);
                    green.set_low();
                    blue.set_low();
                    red.set_high();
                }
                2 => {
                    println!("Green");
                    println!("mode {}", mode);
                    red.set_low();
                    blue.set_low();
                    green.set_high();
                }
                3 => {
                    println!("Blue");
                    println!("mode {}", mode);
                    red.set_low();
                    green.set_low();
                    blue.set_high();
                }
                _ => {}
            }   
        }
        
        last_button_state = button_state;
        
        delay.delay_millis(10);
    }
}