use esp_hal::{
    i2c::master::{Config, I2c},
    peripherals::Peripherals,
    prelude::*,
    uart::{self, Uart},
};
use esp_println::println;

pub struct BoardDrivers<'a> {
    pub display_i2c: I2c<'a, esp_hal::Blocking>,
    pub gps_uart: Uart<'a, esp_hal::Blocking>,
}

pub fn init<'a>(peripherals: Peripherals) -> BoardDrivers<'a> {
    //========================================== I2C DISPLAY SCREEN
    let sda = peripherals.GPIO26;
    let scl = peripherals.GPIO27;

    let i2c_config = esp_hal::i2c::master::Config {
        frequency: 400.kHz(),
        ..Config::default()
    };

    let i2c = I2c::new(peripherals.I2C0, i2c_config)
        .with_sda(sda)
        .with_scl(scl);

    //========================================== UART GPS NEO-6M
    let config = esp_hal::uart::Config::default()
        .baudrate(9600)
        .parity_none()
        .stop_bits(uart::StopBits::STOP1);

    let uart: Uart<'_, esp_hal::Blocking> = match esp_hal::uart::Uart::new_with_config(
        peripherals.UART1,
        config,
        peripherals.GPIO16, //RX
        peripherals.GPIO17, //TX
    ) {
        Ok(uart_instance) => {
            println!("Uart OK");
            uart_instance
        }

        Err(e) => {
            println!("Uart default: {:?}", e);
            loop {}
        }
    };
    //========================================== RETURN
    BoardDrivers {
        display_i2c: i2c,
        gps_uart: uart,
    }
}
