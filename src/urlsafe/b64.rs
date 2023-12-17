use base64::{Engine};
use crate::consts::URLS_B64; 

/// Encodes a given token into a URL-safe Base64 string using the specified Base64 encoding engine.
///
/// # Arguments
///
/// * `token`: A generic type `T` that can be any type convertible to a slice of `u8` bytes. 
///   It represents the token or data that needs to be encoded using URL-safe Base64 encoding.
///
/// # Returns
///
/// A `String` representing the URL-safe Base64 encoding of the input `token`.

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    URLS_B64.encode(token)
}

