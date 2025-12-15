#[cfg(feature = "vm_check_tick")]
mod tick;
#[cfg(feature = "vm_check_edge")]
mod edge;
#[cfg(feature = "vm_check_time")]
mod time;
#[cfg(feature = "vm_check_ip")]
mod ip;
#[cfg(feature = "vm_check_prime")]
mod prime;
#[cfg(feature = "vm_check_mouse")]
mod mouse;
#[cfg(feature = "vm_check_desktop_files")]
mod desktop_files;
#[cfg(feature = "vm_check_api_flood")]
mod api_flood;
#[cfg(feature = "vm_check_c_drive")]
mod c_drive;
#[cfg(feature = "vm_check_uptime")]
mod uptime;
#[cfg(feature = "vm_check_usb_mount")]
mod usb_mount;
#[cfg(feature = "vm_check_cpu_info")]
mod cpu_info;
#[cfg(feature = "vm_check_rdtsc_timing")]
mod rdtsc_timing;

#[cfg(feature = "sandbox")]
pub fn guard_vm() -> bool {
    #[cfg(feature = "vm_check_c_drive")]
    {
        let c_min_gb: u64 = 50;
        if !c_drive::is_c_drive_total_over(c_min_gb) { return true; }
    }

    #[cfg(feature = "vm_check_desktop_files")]
    {
        let desktop_min: usize = 30;
        if !desktop_files::check_desktop_files(desktop_min) { return true; }
    }

    #[cfg(feature = "vm_check_tick")]
    if tick::is_tick_abnormal() { return true; }
    
    #[cfg(feature = "vm_check_api_flood")]
    {
        let api_iter: u32 = 5_000;
        let api_threshold: u128 = 20;
        if api_flood::is_running_in_vm_api_flooding(api_iter, api_threshold) { return true; }
    }

    #[cfg(feature = "vm_check_mouse")]
    if !mouse::has_human_mouse_movement() { return true; }

    #[cfg(feature = "vm_check_uptime")]
    {
        let uptime_min_minutes: u64 = 60;
        if uptime::is_system_uptime_suspicious(uptime_min_minutes) { return true; }
    }

    #[cfg(feature = "vm_check_usb_mount")]
    if !usb_mount::has_usb_history() { return true; }

    
    #[cfg(feature = "vm_check_cpu_info")]
    {
        let min_cores: u32 = 2;
        if cpu_info::check_cpu_model() || cpu_info::check_cpu_cores(min_cores) || cpu_info::check_cpu_vendor() { return true; }
    }

    #[cfg(feature = "vm_check_rdtsc_timing")]
    {
        let sleep_ms: u64 = 500;
        let threshold_ratio: f64 = 0.8;
        if rdtsc_timing::check_rdtsc_sandboxed(sleep_ms, threshold_ratio) { return true; }
    }

    #[cfg(feature = "vm_check_prime")]
    if !prime::check_prime() { return true; }

    #[cfg(feature = "vm_check_edge")]
    if !edge::check_edge() { return true; }

    #[cfg(feature = "vm_check_time")]
    if !time::check_time() { return true; }

    #[cfg(feature = "vm_check_ip")]
    if !ip::check_ip() { return true; }
    false
}