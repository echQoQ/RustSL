use aes_gcm::{Aes256Gcm, Key, Nonce, Tag};
use aes_gcm::aead::{KeyInit, AeadInPlace};
use hkdf::Hkdf;
use p256::{PublicKey, SecretKey};
use sha2::Sha256;

pub fn decrypt(data: &[u8]) -> Result<(*mut u8, usize), Box<dyn std::error::Error>> {
    if data.len() < 32 + 33 + 12 + 16 {
        return Err("Data too short".into());
    }

    let priv_key_bytes = &data[0..32];
    let peer_pub_bytes = &data[32..32+33];
    let nonce = &data[32+33..32+33+12];
    let ciphertext_with_tag = &data[32+33+12..];

    let priv_key = SecretKey::from_bytes(priv_key_bytes.into())
        .map_err(|e| format!("Invalid private key: {}", e))?;

    let peer_pub = PublicKey::from_sec1_bytes(peer_pub_bytes)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    let shared_secret = elliptic_curve::ecdh::diffie_hellman(
        priv_key.to_nonzero_scalar(),
        peer_pub.as_affine()
    );

    let hkdf = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes().as_ref());
    let mut key_bytes = [0u8; 32];
    hkdf.expand(&[], &mut key_bytes).map_err(|_| "HKDF expansion failed")?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce_slice = Nonce::from_slice(nonce);

    let tag_pos = ciphertext_with_tag.len() - 16;
    let ciphertext = &ciphertext_with_tag[..tag_pos];
    let tag = Tag::from_slice(&ciphertext_with_tag[tag_pos..]);

    let plaintext_len = ciphertext.len();

    let ptr = unsafe { crate::alloc_mem::alloc_mem(plaintext_len).map_err(|e| e)? };
    
    if ptr.is_null() {
        return Err("Memory allocation failed".into());
    }

    unsafe {
        std::ptr::copy_nonoverlapping(ciphertext.as_ptr(), ptr, plaintext_len);
    }

    let mut buffer = unsafe { std::slice::from_raw_parts_mut(ptr, plaintext_len) };

    match cipher.decrypt_in_place_detached(nonce_slice, &[], &mut buffer, tag) {
        Ok(_) => {
            Ok((ptr, plaintext_len))
        }
        Err(_) => {
            Err("Decryption failed".into())
        }
    }
}