use crate::domain::DataBrooker;
//================================================================================== CRATES
use esp_hal::i2c::master::I2c;
use embedded_graphics::prelude::*; 
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::Text;
use embedded_graphics::mono_font::iso_8859_15::FONT_9X15_BOLD;
use embedded_graphics::mono_font::MonoTextStyle; 
use heapless::String as HeaplessString;
use esp_println::println;
use core::fmt::Write;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
//================================================================================== ALIASES
pub type DisplayType<'a> = ssd1306::Ssd1306<
    ssd1306::prelude::I2CInterface<esp_hal::i2c::master::I2c<'a, esp_hal::Blocking>>,
    ssd1306::prelude::DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;
//================================================================================== DISPLAYTYPE
pub struct DisplaySetup {
    data1: HeaplessString<32>,
    data2: HeaplessString<32>,
    data3: HeaplessString<32>,
    data4: HeaplessString<32>,
}
//================================================================================== DISPLAYINIT
pub fn display_init<'a>(i2c: I2c<'a, esp_hal::Blocking>) -> DisplayType<'a> {
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    match display.init() {
        Ok(display_instance) => {
            println!("Display OK");
            display_instance
        }
        Err(e) => {
            println!("Display default: {:?}", e);
            loop {}
        }
    };
    display
}
//================================================================================== DISPLAYPRINT

pub fn display_print(
    display_type: &mut DisplayType,
    display_setup: &DisplaySetup,
) {
    match display_type.clear(BinaryColor::Off) {
        Ok(display_clear) => display_clear,
        Err(e) => {
            println!("Display default: {:?}", e);
            loop {}
        }
    };


    let text_style = MonoTextStyle::new(&FONT_9X15_BOLD, BinaryColor::On);

    Text::new(&display_setup.data1, Point { x: 0, y: 12 }, text_style)
        .draw(display_type)
        .unwrap();

    Text::new(&display_setup.data2, Point { x: 0, y: 30 }, text_style)
        .draw(display_type)
        .unwrap();

    Text::new(&display_setup.data3, Point { x: 0, y: 45 }, text_style)
        .draw(display_type)
        .unwrap();

    Text::new(&display_setup.data4, Point { x: 0, y: 60 }, text_style)
        .draw(display_type)
        .unwrap();

    match display_type.flush() {
        Ok(display_instance) => display_instance,
        Err(e) => {
            println!("Display default: {:?}", e);
            loop {}
        }
    };
}

//================================================================================== DISPLAY MODES

pub fn display_mode_1(
    gps_data: &DataBrooker,
) -> DisplaySetup
{
    let mut date: HeaplessString<32> = HeaplessString::new();
    if let Some(naive_date) = gps_data.time_stamp.date {
        let _ = write!(date, "{:?}", naive_date);
    }

    let mut time: HeaplessString<32> = HeaplessString::new();
    if let Some(naive_time) = gps_data.time_stamp.time {
        let _ = write!(time, "{:?}", naive_time);
    }

    let mut speed: HeaplessString<32> = HeaplessString::new();
    if let Some(speed_val) = gps_data.speed.0 {
        let _ = write!(speed, "{:.2?} knots", speed_val);
    }

    let mut voltage: HeaplessString<32> = HeaplessString::new();
    if let Some(volt) = gps_data.voltage {
        let _ = write!(voltage, "{:.2?} volt", volt);
    }

    DisplaySetup{ 
        data1: date, 
        data2: time, 
        data3: speed, 
        data4: voltage
    }
}

// pub fn display_mode_2(
//     gps_data: &DataBrooker,
// ) -> DisplaySetupConstruct
// {
//     let mut time: HeaplessString<32> = HeaplessString::new();
//     if let Some(naive_time) = gps_data.time_stamp.time {
//         let _ = write!(time, "{:?}", naive_time);
//     }

//     let mut speed: HeaplessString<32> = HeaplessString::new();
//     if let Some(speed_val) = gps_data.speed.0 {
//         let _ = write!(speed, "{:.2?} knots", speed_val);
//     }

//     let mut voltage: HeaplessString<32> = HeaplessString::new();
//     if let Some(volt) = gps_data.voltage {
//         let _ = write!(voltage, "{:.2?} volt", volt);
//     }

//     DisplaySetupConstruct { 
//         data1: date, 
//         data2: time, 
//         data3: speed, 
//         data4: voltage
//     }
// }

