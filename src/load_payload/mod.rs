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

#[cfg(all(feature = "load_payload_pipe", not(feature = "load_payload_read_file")))]
pub use pipe::load_payload;

#[cfg(all(feature = "load_payload_mailslot", not(feature = "load_payload_read_file"), not(feature = "load_payload_pipe")))]
pub use mailslot::load_payload;

#[cfg(all(feature = "load_payload_read_file_v2", not(feature = "load_payload_read_file"), not(feature = "load_payload_pipe"), not(feature = "load_payload_mailslot")))]
pub use read_file_v2::load_payload;

#[cfg(all(feature = "load_payload_cmdline", not(feature = "load_payload_read_file"), not(feature = "load_payload_pipe"), not(feature = "load_payload_mailslot"), not(feature = "load_payload_read_file_v2")))]
pub use cmdline::load_payload;

