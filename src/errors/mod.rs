
use std::fmt::{self,Formatter,Display};

#[derive(Debug)]
pub enum OauthError {
    InvalidCodeVerifier,
    InvalidTokenSize, 
    InvalidState,
    InvalidNonce, 
    InvalidPKCEPair,
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
        }
    }
}