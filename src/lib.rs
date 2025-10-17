#[global_allocator]
pub static GLOBAL_ALLOCATOR: &alloc_cat::AllocCat = &alloc_cat::ALLOCATOR;

use base64_simd::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = encode)]
pub fn encode(data: &[u8]) -> String {
    STANDARD.encode_to_string(data)
}

#[wasm_bindgen(js_name = decode)]
pub fn decode(str: &str) -> Result<Vec<u8>, JsValue> {
    STANDARD
        .decode_to_vec(str)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = encodeNoPad)]
pub fn encode_no_pad(data: &[u8]) -> String {
    STANDARD_NO_PAD.encode_to_string(data)
}

#[wasm_bindgen(js_name = decodeNoPad)]
pub fn decode_no_pad(str: &str) -> Result<Vec<u8>, JsValue> {
    STANDARD_NO_PAD
        .decode_to_vec(str)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = encodeUrl)]
pub fn encode_url(data: &[u8]) -> String {
    URL_SAFE.encode_to_string(data)
}

#[wasm_bindgen(js_name = decodeUrl)]
pub fn decode_url(str: &str) -> Result<Vec<u8>, JsValue> {
    URL_SAFE
        .decode_to_vec(str)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = encodeUrlNoPad)]
pub fn encode_url_no_pad(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode_to_string(data)
}

#[wasm_bindgen(js_name = decodeUrlNoPad)]
pub fn decode_url_no_pad(str: &str) -> Result<Vec<u8>, JsValue> {
    URL_SAFE_NO_PAD
        .decode_to_vec(str)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let data = b"hello world";
        let encoded = encode(data);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_without_padding_encoding() {
        let data = b"hello world";
        let encoded = encode_no_pad(data);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ");
        let decoded = decode_no_pad(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_url_encoding() {
        let data = b"hello world";
        let encoded = encode_url(data);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
        let decoded = decode_url(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_url_without_padding_encoding() {
        let data = b"hello world";
        let encoded = encode_url_no_pad(data);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ");
        let decoded = decode_url_no_pad(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
