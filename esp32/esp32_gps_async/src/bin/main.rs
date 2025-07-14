#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]


use esp32_gps_async::domain::TramConstructor;
use esp32_gps_async::parsing::process_received_byte;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::{self, CpuClock};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::uart::{Config, Parity};
use esp_hal::{
    uart::{self, Uart},
};


pub const NMEA_MAX_LEN: usize = 100;
pub const NMEA_TRAM_COUNT: usize = 15;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.4.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    let clocks = CpuClock::max();



    let uart_config = uart::Config::default()
        .with_baudrate(9600)   
        .with_data_bits(uart::DataBits::_8)
        .with_parity(uart::Parity::None)
        .with_stop_bits(uart::StopBits::_1)
        .with_clock_source(uart::ClockSource::Apb)
        ;


    let uart = Uart::new(
        peripherals.UART1,
        uart_config
        )
        .unwrap()
        .with_rx(peripherals.GPIO16)
        .with_tx(peripherals.GPIO17);

// peripherals.GPIO17, //TX
// peripherals.GPIO16, //RX
    let mut uart_async = uart.into_async();

    let mut tram_constructor = TramConstructor::default();
    let mut buffer = [0u8;1];

    loop {
        match uart_async.read_async(&mut buffer).await {
            Ok(byte) => { 
                process_received_byte(byte, &mut tram_constructor).await;

            }
            Err(e) => esp_println::println!("\nError UART: {:?}", e)
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
