use base64::{Engine, engine::{general_purpose,GeneralPurpose}, alphabet};

pub const URLS_B64: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
pub const URL_SAFE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_~"; 
