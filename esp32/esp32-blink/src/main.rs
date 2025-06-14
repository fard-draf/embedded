//! Système de clignotement multi-rythmes
//! Architecture non-bloquante pour détection bouton en temps réel

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{PinDriver, Pull};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;

// Pattern SOS en millisecondes (true = LED ON, false = LED OFF)
const SOS_PATTERN: &[(bool, u32)] = &[
    // S : . . .
    (true, 150),
    (false, 150), // .
    (true, 150),
    (false, 150), // .
    (true, 150),
    (false, 600), // . + pause
    // O : - - -
    (true, 450),
    (false, 150), // -
    (true, 450),
    (false, 150), // -
    (true, 450),
    (false, 600), // - + pause
    // S : . . .
    (true, 150),
    (false, 150), // .
    (true, 150),
    (false, 150), // .
    (true, 150),
    (false, 2000), // . + pause longue avant répétition
];

// Patterns pour modes simples
const SLOW_BLINK: &[(bool, u32)] = &[(true, 1000), (false, 1000)];
const FAST_BLINK: &[(bool, u32)] = &[(true, 200), (false, 200)];
const LED_OFF: &[(bool, u32)] = &[(false, u32::MAX)]; // LED éteinte en permanence

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    // Configuration GPIO
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    let mut button = PinDriver::input(peripherals.pins.gpio0)?;
    button.set_pull(Pull::Up)?;

    // Variables d'état du système
    let mut mode = 1u8;
    let mut last_button_state = true;

    // Variables de timing non-bloquant
    let mut current_time: u32 = 0;
    let mut last_led_toggle: u32 = 0;
    let mut pattern_index: usize = 0;
    let mut led_is_on: bool = false;

    log::info!("Système démarré - Mode 1 (Clignotement lent)");

    loop {
        // Mise à jour du timestamp simulé
        current_time += 10;

        // === DÉTECTION BOUTON ===
        let button_state = button.is_high();

        if button_state != last_button_state && !button_state {
            // Front descendant détecté - changement de mode
            mode = if mode <= 3 { mode + 1 } else { 1 };

            // Log du nouveau mode (une seule fois)
            match mode {
                1 => log::info!("Mode 1 - Clignotement lent"),
                2 => log::info!("Mode 2 - Clignotement rapide"),
                3 => log::info!("Mode 3 - SOS"),
                4 => log::info!("Mode 4 - LED éteinte"),
                _ => {}
            }

            // Reset du pattern pour le nouveau mode
            pattern_index = 0;
            last_led_toggle = current_time;
        }

        last_button_state = button_state;

        // === GESTION DES PATTERNS ===
        let current_pattern = match mode {
            1 => SLOW_BLINK,
            2 => FAST_BLINK,
            3 => SOS_PATTERN,
            _ => LED_OFF, // Mode 4 et valeurs invalides
        };

        // Cas spécial : Mode LED éteinte
        if mode == 4 {
            if led_is_on {
                led.set_low()?;
                led_is_on = false;
            }
        } else {
            // === LOGIQUE DE TIMING NON-BLOQUANTE ===
            let (target_led_state, duration) = current_pattern[pattern_index];

            if current_time - last_led_toggle >= duration {
                // Il est temps de changer l'état de la LED
                led_is_on = target_led_state;

                if led_is_on {
                    led.set_high()?;
                } else {
                    led.set_low()?;
                }

                // Mise à jour pour le prochain changement
                last_led_toggle = current_time;
                pattern_index = (pattern_index + 1) % current_pattern.len();
            }
        }

        // Délai de boucle pour responsivité bouton
        FreeRtos::delay_ms(10);
    }
}
