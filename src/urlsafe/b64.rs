use base64::{Engine};
use crate::consts::URLS_B64; 

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    URLS_B64.encode(token)
}

