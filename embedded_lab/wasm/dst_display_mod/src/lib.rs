#![no_std]
#![crate_type = "cdylib"]

use core::fmt::Write;
use heapless::String;

#[link(wasm_import_module = "env")]
extern "C" {
    fn host_add_line(ptr: *const u8, len: usize);
    fn host_set_line_height(height: u32);
    fn host_clear_layout();
}

#[no_mangle]
pub extern "C" fn generate_layout(speed_knot: f32, voltage: f32, altitude: f32) {
    unsafe { host_clear_layout(); }

    let mut line_1: String<32> = String::new();
    write!(line_1, "Speed: {:.1} knts", speed_knot).unwrap_or_default();
    
    let mut line_2: String<32> = String::new();
    write!(line_2, "Battery: {:.1} volts", voltage).unwrap_or_default();
    
    let mut line_3: String<32> = String::new();
    write!(line_3, "Altitude: {:.1} meters", altitude).unwrap_or_default();


    unsafe {
        host_add_line(line_1.as_ptr(), line_1.len());
        host_add_line(line_2.as_ptr(), line_2.len());
        host_add_line(line_3.as_ptr(), line_3.len());

    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}