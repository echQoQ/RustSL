#[cfg(feature = "load_payload_read_file")]
pub mod read_file;
#[cfg(feature = "load_payload_pipe")]
pub mod pipe;
#[cfg(feature = "load_payload_mailslot")]
pub mod mailslot;
#[cfg(feature = "load_payload_read_file_v2")]
pub mod read_file_v2;
#[cfg(feature = "load_payload_cmdline")]
pub mod cmdline;

#[cfg(feature = "load_payload_read_file")]
pub use read_file::load_payload;

#[cfg(feature = "load_payload_pipe")]
pub use pipe::load_payload;

#[cfg(feature = "load_payload_mailslot")]
pub use mailslot::load_payload;

#[cfg(feature = "load_payload_read_file_v2")]
pub use read_file_v2::load_payload;

#[cfg(feature = "load_payload_cmdline")]
pub use cmdline::load_payload;
