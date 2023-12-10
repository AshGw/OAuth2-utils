use base64::{Engine, engine::{self, general_purpose}, alphabet};

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    _BASE_64.encode(token)
}

const _BASE_64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

