#![no_std]
#![no_main]

// On importe le trait qui fournit la méthode `delay_ms`
use embedded_hal::delay::DelayNs;

use esp_hal::{
    can::{Can, Frame, StandardId},
    clock::CpuClock, // Le chemin pour CpuClock peut être nécessaire
    delay::Delay,
    gpio::Io, // Correction: Io avec un 'i' minuscule
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl // Nouvelle API pour le contrôle système
};
use esp_println::println;
use nb::block;

// Correction: l'attribut est plus simple et fourni par esp_backtrace
use esp_backtrace as _;


#[entry]
fn main() -> ! {
    println!("Démarrage de l'exemple CAN en no_std (API moderne)...");

    // Correction: Nouvelle API d'initialisation
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let system = SystemControl::new(peripherals.SYSTEM);

    let mut delay = Delay::new();

    // Correction: Io avec un 'i' minuscule
    let io = Io::new(peripherals.IO_MUX);

    // Correction: `peripherals.can` en minuscule.
    let mut can = Can::new_blocking_tx(peripherals.can, io.pins.gpio4, io.pins.gpio5);
    
    can.set_bitrate(125_000);
    can.listen();
    
    println!("Driver CAN démarré et à l'écoute.");

    let mut temperature = 24.5_f32;

    loop {
        let data_payload: [u8; 4] = temperature.to_le_bytes();
        let frame_out = Frame::new(StandardId::new(0x123).unwrap(), &data_payload).unwrap();

        block!(can.transmit(&frame_out)).unwrap();
        println!("Message envoyé - ID: 0x123, Temp: {:.1}°C", temperature);

        match block!(can.receive()) {
            Ok(frame_in) => {
                if frame_in.id() == StandardId::new(0x123).unwrap().into() {
                    if let Ok(data) = frame_in.data().try_into() {
                         let temp_in = f32::from_le_bytes(data);
                         println!("Message reçu - ID: 0x123, Temp lue: {:.1}°C", temp_in);
                    }
                }
            }
            Err(e) => {
                println!("Erreur de réception: {:?}", e);
            }
        }
        
        temperature += 0.5;
        if temperature > 40.0 {
            temperature = 20.0;
        }

        // Correction: utilisation de `delay_ms` via le trait DelayNs
        delay.delay_ms(2000_u32);
    }
}