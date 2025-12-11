use rust_veh_syscalls::syscall;
use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::HANDLE;

#[allow(non_snake_case)]
type NtCreateSection = unsafe extern "system" fn(
    SectionHandle: *mut HANDLE,
    DesiredAccess: u32,
    ObjectAttributes: *mut c_void,
    MaximumSize: *mut i64,
    SectionPageProtection: u32,
    AllocationAttributes: u32,
    FileHandle: HANDLE,
) -> i32;

#[allow(non_snake_case)]
type NtMapViewOfSection = unsafe extern "system" fn(
    SectionHandle: HANDLE,
    ProcessHandle: HANDLE,
    BaseAddress: *mut *mut c_void,
    ZeroBits: usize,
    CommitSize: usize,
    SectionOffset: *mut i64,
    ViewSize: *mut usize,
    InheritDisposition: u32,
    AllocationType: u32,
    Win32Protect: u32,
) -> i32;

#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    // Constants
    const SECTION_ALL_ACCESS: u32 = 0xF001F;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    const SEC_COMMIT: u32 = 0x0800_0000;
    // ViewUnmap = 2
    const VIEW_UNMAP: u32 = 2;

    // NtCreateSection via VEH syscall
    let mut section_handle = HANDLE(0);
    let mut max_size: i64 = size as i64;

    let create_status = syscall!(
        "NtCreateSection",
        NtCreateSection,
        &mut section_handle as *mut HANDLE,
        SECTION_ALL_ACCESS,
        ptr::null_mut(),
        &mut max_size as *mut i64,
        PAGE_EXECUTE_READWRITE,
        SEC_COMMIT,
        HANDLE(0)
    );

    if create_status != 0 {
        return Err(format!("NtCreateSection failed: 0x{:x}", create_status));
    }

    // NtMapViewOfSection via VEH syscall
    let mut base_addr: *mut c_void = ptr::null_mut();
    let mut view_size: usize = size;

    let map_status = syscall!(
        "NtMapViewOfSection",
        NtMapViewOfSection,
        section_handle,
        HANDLE(-1isize as isize), // Current process pseudo-handle
        &mut base_addr as *mut *mut c_void,
        0,
        size,
        ptr::null_mut(),
        &mut view_size as *mut usize,
        VIEW_UNMAP,
        0,
        PAGE_EXECUTE_READWRITE
    );

    if map_status == 0 && !base_addr.is_null() {
        Ok(base_addr as *mut u8)
    } else {
        Err(format!("NtMapViewOfSection failed: 0x{:x}", map_status))
    }
}
