pub mod gen;
use crate::consts::CV_DEFAULT_SIZE;
use crate::urlsafe::urlsafe_token;
use gen::gen_code_challenge;

#[derive(Debug, Clone)]
pub struct PKCE {
    pub code_verifier: String,
    pub code_challenge: String,
    pub method: String,
}

impl PKCE {
    /// Generates a new [PKCE](https://tools.ietf.org/html/rfc7636) instance with a randomly generated code verifier,
    /// a derived code challenge, and the specified method (default is "S256").
    pub fn new() -> Self {
        let code_verifier = urlsafe_token(CV_DEFAULT_SIZE);
        let code_challenge = gen_code_challenge(&code_verifier);
        let method = "S256".to_string();

        Self {
            code_verifier,
            code_challenge,
            method,
        }
    }
}
