#[cfg(feature = "vm_check_tick")]
#[allow(dead_code)]
pub fn is_tick_abnormal() -> bool {
    use rustcrypt_ct_macros::{obf_lit_bytes};
    use std::mem::transmute;
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
    unsafe {
        let kernel32 = LoadLibraryA(obf_lit_bytes!(b"kernel32.dll\0").as_ptr());
        if kernel32 == 0 {
            return false;
        }

        let p_get_tick_count = GetProcAddress(kernel32, obf_lit_bytes!(b"GetTickCount\0").as_ptr());
        if p_get_tick_count.is_none() {
            return false;
        }
        let get_tick_count: unsafe extern "system" fn() -> u32 = transmute(p_get_tick_count.unwrap());

        let p_sleep = GetProcAddress(kernel32, obf_lit_bytes!(b"Sleep\0").as_ptr());
        if p_sleep.is_none() {
            return false;
        }
        let sleep: unsafe extern "system" fn(u32) = transmute(p_sleep.unwrap());

        fn simple_rand(seed: &mut u32) -> u32 {
            *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            *seed
        }

        const DELAY_MS: u32 = 200;
        const TIME_TOLERANCE: u32 = 10;

        let mut seed = get_tick_count();
        let random_delay = DELAY_MS.wrapping_add(simple_rand(&mut seed) % 100).wrapping_sub(50);
        let start_tick = get_tick_count();
        sleep(random_delay);
        let end_tick = get_tick_count();
        let mut tick_diff = end_tick.wrapping_sub(start_tick);

        if tick_diff > 10000 {
            tick_diff = (0xFFFFFFFFu32 - start_tick).wrapping_add(end_tick);
        }

        let min_tolerance = TIME_TOLERANCE.wrapping_add(simple_rand(&mut seed) % 30);
        let max_tolerance = TIME_TOLERANCE.wrapping_mul(3).wrapping_add(simple_rand(&mut seed) % 60);

        (tick_diff < random_delay.saturating_sub(min_tolerance)) || (tick_diff > random_delay.saturating_add(max_tolerance))
    }
}