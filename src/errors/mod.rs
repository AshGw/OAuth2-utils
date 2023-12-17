use base64::DecodeError as DecErr;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct TokenError;

#[derive(Debug)]
pub struct NonceError;

#[derive(Debug)]
pub struct StateError;

#[derive(Debug)]
pub enum PKCEError {
    InvalidCodeVerifier,
    InvalidMethod,
    InvalidCodeChallenge,
}

pub type DecodeError = DecErr;

#[derive(Debug)]
pub enum B64Error {
    InvalidEncoding,
    DecodeError,
}

impl Display for B64Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidEncoding => write!(fmt, "Invalid Base64 encoding."),
            Self::DecodeError => write!(fmt, "{}", "Cannot decode he given value",),
        }
    }
}

impl Display for PKCEError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidCodeVerifier => write!(fmt, "Invalid PKCE code verifier."),
            Self::InvalidMethod => write!(fmt, "Invalid PKCE method."),
            Self::InvalidCodeChallenge => write!(fmt, "Invalid PKCE code challenge."),
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Token error.")
    }
}

impl Display for NonceError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Nonce error.")
    }
}

impl Display for StateError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "State error.")
    }
}
