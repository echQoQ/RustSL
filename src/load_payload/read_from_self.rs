pub fn load_payload() -> Result<Vec<u8>, String> {
    const ENCRYPT_DATA: &'static [u8] = include_bytes!("../encrypt.bin");
    Ok(ENCRYPT_DATA.to_vec())
}
