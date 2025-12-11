use rust_veh_syscalls::syscall;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtAllocateVirtualMemory = unsafe extern "system" fn(
    ProcessHandle: HANDLE,
    BaseAddress: *mut *mut c_void,
    ZeroBits: usize,
    RegionSize: *mut usize,
    AllocationType: u32,
    Protect: u32,
) -> i32;

pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    let mut base_address: *mut c_void = ptr::null_mut();
    let mut region_size: usize = size;

    const MEM_COMMIT: u32 = 0x0000_1000;
    const MEM_RESERVE: u32 = 0x0000_2000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    // VEH-based syscall using rust_veh_syscalls
    let status: i32 = syscall!(
        "NtAllocateVirtualMemory",
        NtAllocateVirtualMemory,
        HANDLE(-1isize as isize),          // ProcessHandle
        &mut base_address,                 // BaseAddress
        0,                                 // ZeroBits
        &mut region_size,                  // RegionSize
        MEM_COMMIT | MEM_RESERVE,          // AllocationType
        PAGE_EXECUTE_READWRITE             // Protect
    );

    if status == -1 {
        Err("Failed to resolve syscall for NtAllocateVirtualMemory".to_string())
    } else if status == 0 { // STATUS_SUCCESS
        Ok(base_address as *mut u8)
    } else {
        Err(format!("NtAllocateVirtualMemory failed with status: 0x{:x}", status))
    }
}