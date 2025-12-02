use windows_sys::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
use rustcrypt_ct_macros::obf_lit;

pub unsafe fn load_library(dll_name: &[u8]) -> Result<isize, String> {
    let dll = LoadLibraryA(dll_name.as_ptr() as *const u8);
    if dll == 0 {
        Err(obf_lit!("LoadLibraryA failed").to_string())
    } else {
        Ok(dll)
    }
}

pub unsafe fn get_proc_address(dll: isize, name: &[u8]) -> Result<*const (), String> {
    let addr = GetProcAddress(dll, name.as_ptr() as *const u8);
    if let Some(f) = addr {
        Ok(f as *const ())
    } else {
        Err(obf_lit!("GetProcAddress failed").to_string())
    }
}

pub fn obfuscation_noise() {
    use std::collections::HashMap;
	use rustcrypt_ct_macros::obf_lit_bytes;	
    
    let mut hash_map: HashMap<i32, String> = HashMap::new();
    for i in 0..10 {
        let key = i * 7 + 3;
        let val = format!("value_{}", i);
        hash_map.insert(key, val);
    }
    
    let mut sum: u64 = 0;
    for i in 0..1000 {
        sum = sum.wrapping_add((i * 31 + 17) as u64);
    }
    
    let test_str = obf_lit_bytes!(b"random_buffer_data");
    let mut buffer: Vec<u8> = test_str.iter().map(|&b| b.wrapping_add(5)).collect();
    buffer.reverse();
    let _ = buffer.len();
    
    let _result: Vec<i32> = (0..100)
        .filter(|x| x % 3 == 0)
        .map(|x| x * x)
        .take(20)
        .collect();
    
    use std::time::Instant;
    let _start = Instant::now();
    for _ in 0..100000 {
        let _ = (42i32).wrapping_mul(7);
    }
    
    let mut val: u32 = 0xDEADBEEF;
    for _ in 0..8 {
        val = val.wrapping_shl(1) ^ val.wrapping_shr(3);
    }
    let _ = val;
    
    for (k, v) in hash_map.iter() {
        let _ = format!("{}={}", k, v);
    }
    
    let _ = (0..50).map(|x| x * x).sum::<i32>();
}
