use std::fmt::{self,Formatter,Display};

#[derive(Debug)]
pub enum CodeVerifierError {
        InvalidSize,
}

impl Display for CodeVerifierError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidSize => write!(
                fmt,
                "Invalid code verifier size. It must be between 48 and 128."
            ),
        }
    }
}
