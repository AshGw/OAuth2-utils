use base64::{Engine, engine::{general_purpose,GeneralPurpose}, alphabet};

// push this to consts 
const URLS_B64: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    URLS_B64.encode(token)
}

