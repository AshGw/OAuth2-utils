use crate::consts::URLS_B64;
use crate::errors::B64Error;
use base64::Engine;
use std::borrow::Cow;

/// Encodes a given token into a URL-safe Base64 string using the specified Base64 encoding engine.
pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    URLS_B64.encode(token)
}

type PlainText = Cow<'static, str>;

/// Decodes a URL-safe base64 encoded token
pub fn urlsafe_b64decode<T>(token: T) -> Result<PlainText, B64Error>
where
    T: AsRef<[u8]>,
{
    match URLS_B64.decode(token) {
        Ok(decoded) => {
            Ok(Cow::Owned(String::from_utf8_lossy(&decoded).to_string()))
        }
        Err(_) => Err(B64Error::DecodeError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlsafe_b64encode() {
        // simple string
        let input_string: &str = "Hello, World!";
        let expected_output: &str = "SGVsbG8sIFdvcmxkIQ";
        let encoded_result: String = urlsafe_b64encode(input_string);
        assert_eq!(encoded_result, expected_output);

        // empty str
        let empty_input: &str = "";
        let empty_expected_output: &str = "";
        let empty_encoded_result: String = urlsafe_b64encode(empty_input);
        assert_eq!(empty_encoded_result, empty_expected_output);

        //  binary data
        let binary_data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let binary_expected_output: &str = "AQIDBA";
        let binary_encoded_result: String = urlsafe_b64encode(binary_data);
        assert_eq!(binary_encoded_result, binary_expected_output);
    }

    #[test]
    fn test_urlsafe_b64decode() {
        // Simple string
        let encoded_string: &str = "aGV5eQ";
        let expected_output: &str = "heyy";
        let decoded_result: Result<Cow<'static, str>, B64Error> =
            urlsafe_b64decode(encoded_string);
        assert_eq!(decoded_result.unwrap(), expected_output.to_string());

        // empty str
        let empty_encoded: &str = "";
        let empty_expected_output: &str = "";
        let empty_decoded_result: Result<Cow<'static, str>, B64Error> =
            urlsafe_b64decode(empty_encoded);
        assert_eq!(
            empty_decoded_result.unwrap(),
            empty_expected_output.to_string()
        );

        //  invalid encoding
        let invalid_encoded: &str = "InvalidBase64";
        let invalid_decoded_result: Result<Cow<'static, str>, B64Error> =
            urlsafe_b64decode(invalid_encoded);
        assert!(invalid_decoded_result.is_err());
    }
}
