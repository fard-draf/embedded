use core::{mem, usize};

use crate::display::{DisplayLayout, DisplayType};
use esp_println::println;
use heapless::{String as HeaplessString, Vec};
//==================================================================================
use wasmi::{Engine, Linker, Module, Store, Caller, Func, TypedFunc, Val};
//==================================================================================
pub const WASM_BINARY: &[u8] = include_bytes!("../layout.wasm");
//==================================================================================
pub struct HostState<'a> {
    pub layout: DisplayLayout<'a>
}

pub struct WasmConf<'a> {
    pub layout_func: TypedFunc<(f32,f32,f32), ()>,
    pub store: Store<HostState<'a>>,

}
//==================================================================================

pub fn wasm_init<'a>() -> WasmConf<'a> {
    let engine = Engine::default();
    let mut linker: Linker<HostState> = Linker::new(&engine);

    // host_clear_layout
    linker.func_wrap("env", "host_clear_layout", |mut caller: Caller<'_, HostState>| {
        caller.data_mut().layout.lines.clear();
        println!("WASM -> Cleared layout");
    }).unwrap();

    // host_set_line_height
    linker.func_wrap("env", "host_set_line_height", |mut caller: Caller<'_, HostState>, height: u32| {
        caller.data_mut().layout.line_height = height as usize;
        println!("WASM -> Set line height to {}", height);
    }).unwrap();   
    
    // host_add_line
    linker.func_wrap("env", "host_add_line", |mut caller: Caller<'_, HostState>, ptr: u32, len: u32| {
        const LINE_BUFFER_CAPACITY: usize = 32;
        
        let memory = caller.get_export("memory").and_then(|mem| mem.into_memory()).unwrap();
        let mut buffer: Vec<u8, LINE_BUFFER_CAPACITY> = Vec::new();
        let len_to_read = core::cmp::min(len as usize, LINE_BUFFER_CAPACITY);
        buffer.resize_default(len_to_read).expect("Failed to resize buffer");

        memory.read(&caller, ptr as usize, &mut buffer).unwrap_or_default();
        let line_str = core::str::from_utf8(&buffer).unwrap_or_default();

        let mut heapless_line: HeaplessString<32> = HeaplessString::new();
        heapless_line.push_str(line_str);
        caller.data_mut().layout.lines.push(heapless_line).unwrap();
        
        println!("WASM -> Added line: '{}'", line_str);
    }).unwrap();   

    let module = Module::new(&engine, WASM_BINARY).unwrap();

    let mut host_state = HostState { layout: DisplayLayout::new()};
    let mut store: Store<HostState<'_>> = Store::new(&engine, host_state);

    let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();

    let generate_layout_func: TypedFunc<(f32,f32,f32), ()> = instance
        .get_typed_func(&mut store, "generate_layout")
        .unwrap();

    WasmConf { layout_func: generate_layout_func, store }

}