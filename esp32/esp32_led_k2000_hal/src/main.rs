#![no_std]
#![no_main]


use embedded_hal::{digital::InputPin, pwm::SetDutyCycle};
use esp_backtrace as _;
use esp_hal::{
    prelude::*,delay::Delay, entry, 
    gpio::{self, Input, Io, Level, Output, Pull}, 
    i2c::master::{Config, I2c}, 
    ledc::{self, channel::Number, timer, LSGlobalClkSource, Ledc, LowSpeed}, pcnt::channel, peripheral, 
    prelude::_esp_hal_ledc_timer_TimerIFace, timer::timg::TimerGroup,
    peripherals::Peripherals,
    analog::adc::{Adc, AdcPin, AdcConfig},
};
use esp_println::{print, println};

enum Direction {
    Forward,
    Backward,
}

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    let analog_pin = peripherals.GPIO34;
    let mut adc_conf = AdcConfig::<esp_hal::peripherals::ADC1>::new();
    let mut pin = adc_conf.enable_pin(
        analog_pin, 
        esp_hal::analog::adc::Attenuation::Attenuation11dB
    );

    let mut adc1 = Adc::new(peripherals.ADC1, adc_conf);



    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 10.kHz(),
        })
        .unwrap();
    
    let mut ch0 = ledc.channel::<LowSpeed>(ledc::channel::Number::Channel0, peripherals.GPIO2);
    let mut ch1 = ledc.channel::<LowSpeed>(ledc::channel::Number::Channel1, peripherals.GPIO4);
    let mut ch2 = ledc.channel::<LowSpeed>(ledc::channel::Number::Channel2, peripherals.GPIO5);
    let mut ch3 = ledc.channel::<LowSpeed>(ledc::channel::Number::Channel3, peripherals.GPIO18);
    let mut ch4 = ledc.channel::<LowSpeed>(ledc::channel::Number::Channel4, peripherals.GPIO19);

    
    
    ch0.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0, // On initialise le rapport cyclique à 0%
        pin_config: ledc::channel::config::PinConfig::PushPull,
    }).unwrap();
           
    ch1.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0, // On initialise le rapport cyclique à 0%
        pin_config: ledc::channel::config::PinConfig::PushPull,
    }).unwrap();   
    
    ch2.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0, // On initialise le rapport cyclique à 0%
        pin_config: ledc::channel::config::PinConfig::PushPull,
    }).unwrap();   
    
    ch3.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0, // On initialise le rapport cyclique à 0%
        pin_config: ledc::channel::config::PinConfig::PushPull,
    }).unwrap();   
    
    ch4.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        duty_pct: 0, // On initialise le rapport cyclique à 0%
        pin_config: ledc::channel::config::PinConfig::PushPull,
    }).unwrap();
    
    let channels = (ch0, ch1, ch2, ch3, ch4);
    let max_duty: u32 = channels.0.max_duty_cycle() as u32;


        // --- Variables d'état pour notre machine d'animation ---
    let mut led_brightness: [u32; 5] = [0; 5]; // Stocke la luminosité actuelle de chaque LED
    let mut active_led_index: usize = 0;      // L'index de la LED "tête"
    let mut direction = Direction::Forward;   // La direction actuelle

    // --- Constantes pour régler l'animation ---
    let brightness_step: u32 = max_duty / 25; // Vitesse de fade-in
    let fade_step: u32 = max_duty / 60;       // Vitesse de fade-out (plus lent pour la traîne)

    loop {
        let pot_value = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();

        const MIN_DELAY_US: u32 = 500;
        const MAX_DELAY_US: u32 = 10_000;
        let delay_us = MIN_DELAY_US + (pot_value as u32 * (MAX_DELAY_US - MIN_DELAY_US)) / 4095;

        // --- 1. MISE À JOUR DE L'ÉTAT DE LUMINOSITÉ ---
        for i in 0..5 {
            if i == active_led_index {
                // La LED active s'allume
                led_brightness[i] = led_brightness[i].saturating_add(brightness_step);
            } else {
                // Les autres LEDs s'éteignent (la traîne)
                led_brightness[i] = led_brightness[i].saturating_sub(fade_step);
            }
            // On s'assure de ne pas dépasser la luminosité maximale
            led_brightness[i] = led_brightness[i].min(max_duty);
        }

        // --- 2. APPLICATION DE L'ÉTAT AU MATÉRIEL ---
        // On met à jour toutes les LEDs en une seule fois
        channels.0.set_duty_hw(led_brightness[0]);
        channels.1.set_duty_hw(led_brightness[1]);
        channels.2.set_duty_hw(led_brightness[2]);
        channels.3.set_duty_hw(led_brightness[3]);
        channels.4.set_duty_hw(led_brightness[4]);

        // --- 3. MISE À JOUR DE LA LOGIQUE D'ANIMATION ---
        // Si la LED active a atteint sa pleine luminosité, on passe à la suivante
        if led_brightness[active_led_index] >= max_duty {
            match direction {
                Direction::Forward => {
                    if active_led_index < 4 {
                        active_led_index += 1;
                    } else {
                        // On a atteint la fin, on repart en arrière
                        direction = Direction::Backward;
                        active_led_index -= 1;
                    }
                }
                Direction::Backward => {
                    if active_led_index > 0 {
                        active_led_index -= 1;
                    } else {
                        // On a atteint le début, on repart en avant
                        direction = Direction::Forward;
                        active_led_index += 1;
                    }
                }
            }
        }

        // --- 4. DÉLAI ---
        // Petite pause pour contrôler la vitesse globale de l'animation
        delay.delay_micros(delay_us);
    }
}

