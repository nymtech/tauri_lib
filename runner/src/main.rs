use libloading::{Library, Symbol};

fn call_dynamic() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new("libvpnym.so")?;
        // let lib = Library::new("libvpnym.so")?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"run_tauri")?;
        func();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    call_dynamic()
}
