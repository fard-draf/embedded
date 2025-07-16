
use std::fs;
use wasmi::{Engine, Module};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_path = "target/wasm32-unknown-unknown/release/dst_display_mod.wasm";
    let aot_output_path = "applications/esp32/hal/dinghy_sail_tools/wasm_modules/display_module.aot";

    println!("---> Reading Wasm from: {}", wasm_path);
    // L'opérateur `?` est crucial ici.
    // Si fs::read réussit, wasm_bytes sera un `Vec<u8>`.
    // Si ça échoue, la fonction main retournera l'erreur.
    let wasm_bytes = fs::read(wasm_path)?;

    println!("---> Compiling Wasm module...");
    let engine = Engine::default();
    // On passe `&wasm_bytes` qui est maintenant bien un `&[u8]`.
    // Le `?` "déballe" le Result retourné par Module::new.
    let module = Module::new(&engine, &wasm_bytes)?;

    println!("---> Serializing compiled module to: {}", aot_output_path);
    // La méthode `to_bytes` existe bien sur le type `Module`.
    // L'erreur précédente venait du fait que `module` était un `Result`.
    // Le `?` gère l'échec possible de la sérialisation.
    let serialized_bytes = module.()?;

    fs::write(aot_output_path, serialized_bytes)?;

    println!("\n✅ Successfully created AOT module at '{}'" , aot_output_path);
    Ok(())
}
