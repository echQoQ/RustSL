#[allow(dead_code)]
pub fn is_system_uptime_suspicious(min_minutes: u64) -> bool {
    use rustcrypt_ct_macros::obf_lit_bytes;
    use std::mem::transmute;
    use crate::utils::{load_library, get_proc_address};
    
    unsafe {
        let kernel32 = match load_library(&obf_lit_bytes!(b"kernel32.dll\0")) {
            Ok(lib) => lib,
            Err(_) => return false,
        };
        
        let p_tick64 = match get_proc_address(kernel32, &obf_lit_bytes!(b"GetTickCount64\0")) {
            Ok(f) => Some(f),
            Err(_) => None,
        };
        if let Some(f) = p_tick64 {
            let get_tick64: unsafe extern "system" fn() -> u64 = transmute(f);
            let uptime_ms = get_tick64();
            let uptime_minutes = uptime_ms / 60_000;
            
            // 系统运行时间小于阈值视为可疑（沙箱通常刚启动快照）
            return uptime_minutes < min_minutes;
        }
        
        // 降级到 GetTickCount（32位，约49天溢出）
        let p_tick = match get_proc_address(kernel32, &obf_lit_bytes!(b"GetTickCount\0")) {
            Ok(f) => Some(f),
            Err(_) => None,
        };
        if let Some(f) = p_tick {
            let get_tick: unsafe extern "system" fn() -> u32 = transmute(f);
            let uptime_ms = get_tick() as u64;
            let uptime_minutes = uptime_ms / 60_000;
            return uptime_minutes < min_minutes;
        }
        
        false
    }
}