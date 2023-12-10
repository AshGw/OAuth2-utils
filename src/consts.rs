use base64::{Engine, engine::{self, general_purpose, GeneralPurpose}, alphabet};

pub const DEFAULT_TOKEN_SIZE: usize = 96; 
pub const GP_URLSAFE_BASE_64: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);




