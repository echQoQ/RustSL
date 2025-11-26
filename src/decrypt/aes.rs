use crate::alloc_mem::alloc_mem;
pub unsafe fn decrypt(decoded: &[u8]) -> Result<(usize, usize), String> {
    use rustcrypt_ct_macros::obf_lit;
    use aes::Aes256;
    use cipher::{BlockDecryptMut, KeyIvInit, block_padding::Pkcs7};
    use sha2::{Sha256, Digest};

    type Aes256CbcDec = cbc::Decryptor<Aes256>;
    
    let key_len = 32;  // AES-256 key size
    let iv_len = 16;   // AES block size
    let hash_len = 32; // SHA-256 hash size
    
    if decoded.len() < key_len + iv_len + hash_len + 1 {
        return Err(obf_lit!("aes payload too short").to_string());
    }
    
    let key = &decoded[0..key_len];
    let iv = &decoded[key_len..key_len + iv_len];
    let hash = &decoded[key_len + iv_len..key_len + iv_len + hash_len];
    let encrypted = &decoded[key_len + iv_len + hash_len..];
    
    let p = unsafe { alloc_mem(encrypted.len())? };
    std::ptr::copy_nonoverlapping(encrypted.as_ptr(), p, encrypted.len());
    let buf = std::slice::from_raw_parts_mut(p, encrypted.len());
    
    let cipher = Aes256CbcDec::new_from_slices(key, iv)
        .map_err(|_| obf_lit!("invalid aes key or iv").to_string())?;
        
    let pt_len = cipher.decrypt_padded_mut::<Pkcs7>(buf)
        .map_err(|_| obf_lit!("aes decryption failed").to_string())?
        .len();
    
    // Calculate hash of decrypted data
    let mut hasher = Sha256::new();
    hasher.update(&buf[..pt_len]);
    let calc_hash = hasher.finalize();
    
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("aes hash mismatch").to_string());
    }
    
    Ok((p as usize, pt_len))
}