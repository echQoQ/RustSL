#[cfg(feature = "load_payload_read_from_self")]
pub mod read_from_self;
#[cfg(feature = "load_payload_pipe")]
pub mod pipe;

#[cfg(feature = "load_payload_read_from_self")]
pub use read_from_self::load_payload;

#[cfg(all(feature = "load_payload_pipe", not(feature = "load_payload_read_from_self")))]
pub use pipe::load_payload;

