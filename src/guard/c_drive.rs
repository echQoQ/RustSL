#[cfg(feature = "vm_check_c_drive")]
#[allow(dead_code)]
pub fn is_c_drive_total_over(threshold_gb: u64) -> bool {
    use std::mem::transmute;
    use rustcrypt_ct_macros::{obf_lit_bytes};
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
    unsafe {
        let kernel32 = LoadLibraryA(obf_lit_bytes!(b"kernel32.dll\0").as_ptr());
        if kernel32 == 0 {
            return false;
        }

        let p_gdse = GetProcAddress(kernel32, obf_lit_bytes!(b"GetDiskFreeSpaceExA\0").as_ptr());
        let p_gdse = match p_gdse {
            Some(f) => f,
            None => return false,
        };

        let gdse: unsafe extern "system" fn(*const u8, *mut u64, *mut u64, *mut u64) -> i32 = transmute(p_gdse);

        let mut free_avail: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut total_free: u64 = 0;

        let root = obf_lit_bytes!(b"C:\\\0");
        let ok = gdse(root.as_ptr(), &mut free_avail as *mut u64, &mut total_bytes as *mut u64, &mut total_free as *mut u64);
        if ok == 0 {
            return false;
        }

        return total_bytes >= threshold_gb.saturating_mul(1024 * 1024 * 1024)
    }
}