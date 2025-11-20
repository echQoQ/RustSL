use crate::utils::{load_library, get_proc_address};

// alloc_mem: VirtualAlloc实现
#[cfg(feature = "alloc_mem_va")]
#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::obf_lit;
    use core::ffi::c_void;

    // 定义函数指针类型
    type VirtualAllocFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_allocation_type: u32, fl_protect: u32) -> *mut c_void;

    // 加载 kernel32.dll
    let kernel32 = load_library(b"kernel32.dll\0")?;
    let virtual_alloc: VirtualAllocFn = core::mem::transmute(get_proc_address(kernel32, b"VirtualAlloc\0")?);

    let p = virtual_alloc(core::ptr::null_mut(), size, 0x00001000, 0x40) as *mut u8; // MEM_COMMIT, PAGE_EXECUTE_READWRITE
    if p.is_null() {
        return Err(obf_lit!("VirtualAlloc failed").to_string());
    }
    Ok(p)
}