use base64::{engine::general_purpose as b64, Engine as _};

use crate::Error;

pub fn encode_bytes(bytes: &[u8]) -> String {
    b64::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn decode_string(s: &str) -> Result<Vec<u8>, Error> {
    Ok(b64::URL_SAFE_NO_PAD.decode(s)?)
}