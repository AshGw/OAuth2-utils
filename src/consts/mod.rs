use base64::{Engine, engine::{general_purpose,GeneralPurpose}, alphabet};

pub const URLS_B64: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
pub const URL_SAFE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_~"; 

fn urlsafe_chars() -> String {
    (b'A'..=b'Z')
        .map(char::from)
        .chain((b'a'..=b'z').map(char::from))
        .chain((b'0'..=b'9').map(char::from))
        .chain("-_~".chars())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlsafe_chars() {
        assert_eq!(urlsafe_chars(), URL_SAFE_CHARS);
    }
}