#![allow(dead_code, unused_imports)]

mod http;
pub use http::http_get;

mod win_api;
pub use win_api::{
    load_library,
    get_proc_address
};

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
pub use debug::{
    print_message,
    print_error
};


pub fn simple_decrypt(encrypted: &str) -> String {
    use obfstr::obfbytes;
    use base64::{Engine as _, engine::general_purpose};
    let decoded = general_purpose::STANDARD.decode(encrypted).unwrap();
    let obf_key = obfbytes!(b"rsl_secret_key_2025");
    let key = obf_key.as_slice();
    let decrypted: Vec<u8> = decoded.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect();
    String::from_utf8(decrypted).unwrap()
}
