use std::str::Bytes;

use base64::{Engine as _, engine::{self, general_purpose}, alphabet};


pub const BASE_64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);


    pub fn base64_encode<T>(token: T) -> String
    where
        T: AsRef<[u8]>,
    {
        BASE_64.encode(token)
    }