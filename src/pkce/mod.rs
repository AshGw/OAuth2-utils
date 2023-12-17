pub mod error;
pub mod gen;

pub use error::CodeVerifierError;
use crate::urlsafe::urlsafe_token;
use gen::gen_code_challenge;


#[derive(Debug, Clone)]
pub struct PKCE {
    pub code_verifier: String,
    pub code_challenge: String,
    pub method: String,
}

// consts form the consts module for 96
impl PKCE {
    pub fn new() -> Self {
        let code_verifier = urlsafe_token(96);
        let code_challenge = gen_code_challenge(&code_verifier);
        let method = "S256".to_string(); 

        Self {
            code_verifier,
            code_challenge,
            method
        }
    }
}
