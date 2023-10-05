use libloading::{Library, Symbol};

#[cfg(target_os = "linux")]
const LIB_FILE: &str = "libvpnym.so";
#[cfg(target_os = "windows")]
const LIB_FILE: &str = "vpnym.dll";
#[cfg(target_os = "macos")]
const LIB_FILE: &str = "vpnym.dylib";

fn call_dynamic() -> Result<(), Box<dyn std::error::Error>> {
    let lib_path = format!("lib/{LIB_FILE}");
    println!("lib_path: {}", lib_path);
    unsafe {
        let lib = Library::new(lib_path)?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"run_tauri")?;
        func();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    call_dynamic()
}
