use crate::gen::gen_code_challenge;
use crate::urlsafe::urlsafe_token;


#[derive(Debug, Clone)]
pub struct PKCE {
    pub code_verifier: String,
    pub code_challenge: String,
    pub method: String,
}

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
