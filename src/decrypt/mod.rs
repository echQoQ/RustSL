// IPv4 decryption
#[cfg(feature = "decrypt_ipv4")]
mod ipv4;
#[cfg(feature = "decrypt_ipv4")]
pub use crate::decrypt::ipv4::decrypt;

// IPv6 decryption
#[cfg(feature = "decrypt_ipv6")]
mod ipv6;
#[cfg(feature = "decrypt_ipv6")]
pub use crate::decrypt::ipv6::decrypt;

// UUID decryption
#[cfg(feature = "decrypt_uuid")]
mod uuid;
#[cfg(feature = "decrypt_uuid")]
pub use crate::decrypt::uuid::decrypt;

// MAC decryption
#[cfg(feature = "decrypt_mac")]
mod mac;
#[cfg(feature = "decrypt_mac")]
pub use crate::decrypt::mac::decrypt;

// RC4 decryption
#[cfg(feature = "decrypt_rc4")]
mod rc4;
#[cfg(feature = "decrypt_rc4")]
pub use crate::decrypt::rc4::decrypt;

// AES decryption
#[cfg(feature = "decrypt_aes")]
mod aes;
#[cfg(feature = "decrypt_aes")]
pub use crate::decrypt::aes::decrypt;

// XChaCha20 decryption
#[cfg(feature = "decrypt_xchacha20")]
mod xchacha20;
#[cfg(feature = "decrypt_xchacha20")]
pub use crate::decrypt::xchacha20::decrypt;