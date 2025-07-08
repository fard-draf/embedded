#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    clock,
    gpio::{Input, Io, Level, Output, Pull},
    i2c::master::{Config, I2c},
    prelude::*
    
};
use esp_println::{print, println};
use bme280::{i2c::{self, BME280}, Measurements, SensorMode};


#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Status {
    Ok = 0x00,
    TempError = 0x01,
    PressureError = 0x02,
    GlobalError = 0x03,
    Empty = 0x04

}

#[repr(C)]
#[derive(Debug)]
pub struct SensorPacket {
    start_byte: u8, //1 bytes - OFFSET 0 PADDING AVNT 0
    checksum: u8, // 1 byte - OS 1 PA 0
    status: Status, // 1 byte - OS 2 PA 0 
    temperature: f32, // 4 bytes - OS 4 PA 1
    pressure: f32, // 4 bytes -  OS 8 PA 0
}

// non opti Raw memory packet [170, 0, 22, 189, 59, 195, 184, 65, 48, 77, 199, 71, 73, 127, 203, 149]
// opti     Raw memory packet [58, 228, 184, 65, 221, 78, 199, 71, 170, 0, 158, 149]


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

        let mut packet = SensorPacket {
            start_byte: 0xAA,
            status: Status::Empty,
            temperature: 0.0,
            pressure: 0.0,
            checksum: 0,
        };

        match bme280.measure(&mut delay) {
            Ok(measurements) => {
                packet.temperature = measurements.temperature;
                packet.pressure = measurements.pressure;
                packet.status = Status::Ok;
            }
            Err(e) => {
                packet.status = Status::GlobalError;
            }
        }

        let packed_bytes = unsafe {
            core::slice::from_raw_parts(&packet as *const SensorPacket as *const u8, size_of::<SensorPacket>())
        };

        packet.checksum = packed_bytes[..packed_bytes.len() - 1]
            .iter()
            .fold(0, |acc, &x| acc ^ x);


        println!("Packet size: {} bytes", size_of::<SensorPacket>());
        println!("Raw memory packet {:?}", packed_bytes);
        
        led.set_high();
        delay.delay_millis(100);
        
        
        led.set_low();
        delay.delay_millis(4000);

    }


}