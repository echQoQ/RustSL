use crate::utils::{load_library, get_proc_address};
use rustcrypt_ct_macros::obf_lit_bytes;

pub fn check() -> Result<(), String> {
    unsafe {
        let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice())?;
        let user32 = load_library(obf_lit_bytes!(b"user32.dll\0").as_slice())?;

        // Define function types
        type GetCurrentThreadIdFn = unsafe extern "system" fn() -> u32;
        type PostThreadMessageAFn = unsafe extern "system" fn(u32, u32, usize, isize) -> i32;
        type PeekMessageAFn = unsafe extern "system" fn(*mut MSG, isize, u32, u32, u32) -> i32;

        // Load functions
        let get_current_thread_id: GetCurrentThreadIdFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetCurrentThreadId\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load GetCurrentThreadId\0").as_slice()).to_string())?);

        let post_thread_message_a: PostThreadMessageAFn = std::mem::transmute(get_proc_address(user32, obf_lit_bytes!(b"PostThreadMessageA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load PostThreadMessageA\0").as_slice()).to_string())?);
        let peek_message_a: PeekMessageAFn = std::mem::transmute(get_proc_address(user32, obf_lit_bytes!(b"PeekMessageA\0").as_slice()).map_err(|_| String::from_utf8_lossy(obf_lit_bytes!(b"Failed to load PeekMessageA\0").as_slice()).to_string())?);

        #[repr(C)]
        #[allow(non_snake_case)]
        struct MSG {
            hwnd: isize,
            message: u32,
            wParam: usize,
            lParam: isize,
            time: u32,
            pt_x: i32,
            pt_y: i32,
        }

        let mut msg: MSG = std::mem::zeroed();
        const WM_USER: u32 = 0x0400;
        
        post_thread_message_a(get_current_thread_id(), WM_USER + 2, 23, 42);
        
        // PeekMessage with PM_NOREMOVE (0)
        if peek_message_a(&mut msg, -1, 0, 0, 0) == 0 {
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"PeekMessage failed or no message found (Sandbox detected?)\0").as_slice()).to_string());
        }

        if msg.message != WM_USER + 2 || msg.wParam != 23 || msg.lParam != 42 {
            return Err(String::from_utf8_lossy(obf_lit_bytes!(b"Message content mismatch (Sandbox detected?)\0").as_slice()).to_string());
        }

        Ok(())
    }
}
