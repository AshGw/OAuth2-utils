use base64::{
    alphabet,
    engine::{general_purpose, GeneralPurpose},
};

pub const URLS_B64: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
pub const URL_SAFE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_~";
// CV is short for code verifier 
pub const CV_DEFAULT_SIZE: usize = 96;
pub const CV_MIN_SIZE: usize = 43;
pub const CV_MAX_SIZE: usize = 128;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlsafe_chars() {
        let chars: String = (b'A'..=b'Z')
            .map(char::from)
            .chain((b'a'..=b'z').map(char::from))
            .chain((b'0'..=b'9').map(char::from))
            .chain("-_~".chars())
            .collect();
        assert_eq!(chars, URL_SAFE_CHARS);
    }
}
