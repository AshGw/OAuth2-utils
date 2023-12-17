use std::fmt::{self,Formatter,Display};
use base64::DecodeError;

#[derive(Debug)]
pub enum OauthError {
    InvalidCodeVerifier,
    InvalidTokenSize, 
    InvalidState,
    InvalidNonce, 
    InvalidPKCEPair,
    B64DecodeError(DecodeError),
}

impl Display for OauthError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidCodeVerifier => write!(
                fmt,
                "..."
            ),
            Self::InvalidTokenSize => write!(
                fmt,
                "..."
            ),
            Self::InvalidState => write!(
                fmt,
                "..."
            ),
            Self::InvalidNonce => write!(
                fmt,
                "..."
            ),
            Self::InvalidPKCEPair => write!(
                fmt,
                "..."
            ),
            Self::B64DecodeError(err) => write!(
                fmt,
                "{}", err
            ),
        }
    }
}