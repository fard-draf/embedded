#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    i2c::master::{Config, I2c},
    prelude::*,
};
use esp_println::{print, println};
use bme280::{i2c::{self, BME280}, Measurements, SensorMode};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut delay = Delay::new();
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    // let io = Io::new(peripherals.IO_MUX);
    
    let sda = peripherals.GPIO21;
    let scl = peripherals.GPIO22;
    
    let i2c_config = Config {
        frequency: 100.kHz(),
        ..Config::default()
    };

    let mut i2c: I2c<'_, esp_hal::Blocking> = I2c::new(peripherals.I2C0, i2c_config)
        .with_sda(sda)
        .with_scl(scl);

    println!("Initialisation du BME280...");
    

    let mut bme280 = BME280::new_primary(i2c);
    
    match bme280.init(&mut delay) {
        Ok(_) => println!("BME280 initialisé avec succès"),
        Err(e) => {
            println!("Erreur d'initialisation: {:?}", e);
            println!("Vérifiez les connexions et l'adresse I2C (0x76/0x77)");
            panic!("BME280 non trouvé!");
        }
    }
    


    println!("Démarrage des lectures...\n");
    
    
    loop {
        
        match bme280.measure(&mut delay) {
            Ok(measurements) => {
                println!("=== Mesures BME280 ===");
                println!("Température: {:.2}°C", measurements.temperature);
                // println!("Humidité:    {:.2}%", measurements.humidity);
                println!("Pression:    {:.2} hPa", measurements.pressure / 100.0);
                println!("=====================\n");
            }
            Err(e) => {
                println!("Erreur de lecture: {:?}", e);
            }
        }
        
        led.set_high();
        delay.delay_millis(100);
        
        
        led.set_low();
        delay.delay_millis(10000);

    }


}