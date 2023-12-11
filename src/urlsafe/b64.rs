use base64::{Engine, engine::{self, general_purpose}, alphabet};
use sha2::{Digest, Sha256};

const B64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    B64.encode(token)
}

