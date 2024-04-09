use crate::consts::{CV_MAX_SIZE, CV_MIN_SIZE};
use base64::DecodeError as DecErr;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum CodeVerifierError {
    TooBig,
    TooSmall,
}

pub type DecodeError = DecErr;

#[derive(Debug, PartialEq)]
pub enum B64Error {
    InvalidEncoding,
    DecodeError,
}

impl Display for B64Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidEncoding => write!(fmt, "Invalid Base64 encoding."),
            Self::DecodeError => write!(fmt, "Cannot decode the given value"),
        }
    }
}

impl Display for CodeVerifierError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::TooBig => write!(fmt, "It must be less than {}", CV_MAX_SIZE),
            Self::TooSmall => {
                write!(fmt, "It must be greater than {}", CV_MIN_SIZE)
            }
        }
    }
}

impl std::error::Error for CodeVerifierError {}
impl std::error::Error for B64Error {}
