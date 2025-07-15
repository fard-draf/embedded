use crate::domain::DataBrooker;
//================================================================================== CRATES
use esp_hal::i2c::master::I2c;
use embedded_graphics::prelude::*; 
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::Text;
use embedded_graphics::mono_font::iso_8859_15::FONT_9X15_BOLD;
use embedded_graphics::mono_font::iso_8859_14::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle; 
use heapless::{String as HeaplessString, Vec};
use esp_println::println;
use core::fmt::Write;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
//================================================================================== ALIASES
pub type DisplayType<'a> = ssd1306::Ssd1306<
    ssd1306::prelude::I2CInterface<esp_hal::i2c::master::I2c<'a, esp_hal::Blocking>>,
    ssd1306::prelude::DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
>;
//================================================================================== MAX_LINES_SCREEN 
const MAX_LINES_ON_SCREEN: usize = 4;
//================================================================================== DISPLAYTYPE

#[derive(Debug)]
pub struct DisplayLayout<'a> {
    pub lines: Vec<HeaplessString<32>, MAX_LINES_ON_SCREEN>,
    pub text: MonoTextStyle<'a, BinaryColor>,
    pub line_height: usize,

}

impl<'a> DisplayLayout<'a> {
    pub fn new() -> Self {
        Self { 
            lines: Vec::new(),
            text: MonoTextStyle::new(&FONT_9X15_BOLD, BinaryColor::On),
            line_height: 12,
        }
    }
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
    display: &mut DisplayType,
    layout: &DisplayLayout,
) {
    match display.clear(BinaryColor::Off) {
        Ok(display_clear) => display_clear,
        Err(e) => {
            println!("Display default: {:?}", e);
            loop {}
        }
    };


    let text_style = layout.text;
    let line_height = layout.line_height as i32;

    for (index, text_line) in layout.lines.iter().enumerate() {
        let y_position = 12 + (index as i32 * line_height);
        Text::new(&text_line, Point { x: 0, y: y_position }, text_style)
            .draw(display).unwrap();
    }


    match display.flush() {
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
) -> DisplayLayout
{
    let mut layout = DisplayLayout::new();

    if let Some(naive_time) = gps_data.time_stamp.time {
    let mut time: HeaplessString<32> = HeaplessString::new();
        let _ = write!(time, "{:?}", naive_time);
        layout.lines.push(time).unwrap_or_default();
    }

    if let Some(speed_val) = gps_data.speed.0 {
    let mut speed: HeaplessString<32> = HeaplessString::new();
        let _ = write!(speed, "{:.2?} knots", speed_val);
        layout.lines.push(speed).unwrap_or_default();
    }

    if let Some(volt) = gps_data.voltage {
    let mut voltage: HeaplessString<32> = HeaplessString::new();
        let _ = write!(voltage, "{:.2?} volt", volt);
        layout.lines.push(voltage).unwrap_or_default();
    }

    layout.line_height = 15;
    layout.text = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    layout


}


pub fn display_mode_2(
    gps_data: &DataBrooker,
) -> DisplayLayout
{
    let mut layout = DisplayLayout::new();

    if let Some(speed_val) = gps_data.speed.0 {
    let mut speed: HeaplessString<32> = HeaplessString::new();
        let _ = write!(speed, "speed:  {:.2?}", speed_val);
        layout.lines.push(speed).unwrap_or_default();
    }

    if let Some(alt) = gps_data.altitude {
    let mut altitude: HeaplessString<32> = HeaplessString::new();
        let _ = write!(altitude, "alt:    {:?}", alt);
        layout.lines.push(altitude).unwrap_or_default();
    }

    if let Some(volt) = gps_data.voltage {
    let mut voltage: HeaplessString<32> = HeaplessString::new();
        let _ = write!(voltage, "volt:   {:.2?}", volt);
        layout.lines.push(voltage).unwrap_or_default();
    }

    layout.line_height = 22;
    layout.text = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    layout


}
