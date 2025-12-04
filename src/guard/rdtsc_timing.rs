use std::thread;
use std::time::Duration;

#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[allow(dead_code)]
fn rdtsc() -> u64 {
    unsafe {
        let low: u32;
        let high: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack, preserves_flags)
        );
        ((high as u64) << 32) | (low as u64)
    }
}

#[cfg(target_arch = "x86")]
#[inline(never)]
#[allow(dead_code)]
fn rdtsc() -> u64 {
    unsafe {
        let low: u32;
        let high: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack, preserves_flags)
        );
        ((high as u64) << 32) | (low as u64)
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
#[allow(dead_code)]
fn rdtsc() -> u64 {
    0
}

/// Checks for time acceleration using RDTSC
/// Returns true if execution appears to be accelerated (sandbox behavior)
/// 
/// # Parameters
/// * `sleep_ms` - Sleep duration in milliseconds
/// * `threshold_ratio` - Threshold ratio (0.0-1.0), e.g., 0.8 means 80% of expected cycles
#[allow(dead_code)]
pub fn check_rdtsc_sandboxed(sleep_ms: u64, threshold_ratio: f64) -> bool {
    let tsc1 = rdtsc();
    thread::sleep(Duration::from_millis(sleep_ms));
    let tsc2 = rdtsc();
    let tsc_delta = tsc2.saturating_sub(tsc1);
    let expected_cycles_per_ms = 3_000_000u64;
    let expected_delta = sleep_ms * expected_cycles_per_ms;
    
    //print!("RDTSC delta: {}, expected delta: {}\n", tsc_delta, expected_delta);

    // Convert to float for percentage calculation
    let tsc_delta_f = tsc_delta as f64;
    let expected_delta_f = expected_delta as f64;
    
    // If actual cycles < threshold_ratio of expected, time has been accelerated (sandbox)
    let threshold = expected_delta_f * threshold_ratio;
    //print!("RDTSC threshold ({}%): {}\n", threshold_ratio * 100.0, threshold);
    
    tsc_delta_f < threshold
}
