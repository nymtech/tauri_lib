use libloading::{Library, Symbol};
use std::env::consts::OS;

fn call_dynamic() -> Result<(), Box<dyn std::error::Error>> {
    let lib_file = match OS {
        "windows" => "vpnym.dll",
        "macos" => "vpnym.dylib",
        _ => "libvpnym.so",
    };
    let lib_path = format!("lib/{lib_file}");
    println!("lib_path: {}", lib_path);
    unsafe {
        let lib = Library::new(lib_path)?;
        // let lib = Library::new("libvpnym.so")?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"run_tauri")?;
        func();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    call_dynamic()
}
