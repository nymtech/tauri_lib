use libloading::{Library, Symbol};

fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new("../app/src-tauri/target/release/libvpnym.so")?;
        // let lib = Library::new("../app/src-tauri/target/debug/libvpnym.so")?;
        let func: Symbol<unsafe extern fn() -> u32> = lib.get(b"run_tauri")?;
        Ok(func())
    }
}

fn main() {
    call_dynamic().ok();
}
