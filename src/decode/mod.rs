
#[cfg(feature = "base32_decode")]
#[allow(dead_code)]
pub fn decode_payload(data: &[u8]) -> Option<Vec<u8>> {
    let raw = std::str::from_utf8(data).ok()?;
    base32::decode(base32::Alphabet::Rfc4648 { padding: true }, raw)
}

#[cfg(feature = "base64_decode")]
#[allow(dead_code)]
pub fn decode_payload(data: &[u8]) -> Option<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.decode(data).ok()
}

#[cfg(feature = "urlsafe_base64_decode")]
#[allow(dead_code)]
pub fn decode_payload(data: &[u8]) -> Option<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE.decode(data).ok()
}

#[cfg(feature = "hex_decode")]
#[allow(dead_code)]
pub fn decode_payload(data: &[u8]) -> Option<Vec<u8>> {
    let raw = std::str::from_utf8(data).ok()?;
    hex::decode(raw.trim()).ok()
}

#[cfg(feature = "none_decode")]
#[allow(dead_code)]
pub fn decode_payload(data: &[u8]) -> Option<Vec<u8>> {
	Some(data.to_vec())
}