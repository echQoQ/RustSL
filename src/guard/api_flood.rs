#[allow(dead_code)]
use obfstr::obfbytes;
pub fn is_running_in_vm_api_flooding(iterations: u32, threshold_ms: u128) -> bool {
    use std::time::Instant;
    use std::mem::transmute;
    use crate::utils::{load_library, get_proc_address};

    unsafe {
        let kernel32 = match load_library(obfbytes!(b"kernel32.dll\0").as_slice()) {
            Ok(lib) => lib,
            Err(_) => return false,
        };

        // Resolve GetSystemTimeAsFileTime to repeatedly call and measure
        let p_time = match get_proc_address(kernel32, obfbytes!(b"GetSystemTimeAsFileTime\0").as_slice()) {
            Ok(f) => f,
            Err(_) => return false,
        };

        // typedef VOID (WINAPI *GetSystemTimeAsFileTime)(LPFILETIME);
        let get_time: unsafe extern "system" fn(*mut u64) = transmute(p_time);

        let mut filetime: u64 = 0;
        let start = Instant::now();
        for _ in 0..iterations {
            get_time(&mut filetime as *mut u64);
        }
        let elapsed = start.elapsed().as_millis();

        elapsed > threshold_ms
    }
}