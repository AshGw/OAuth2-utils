mod urlsafe;

use urlsafe::{token_urlsafe, urlsafe_b64encode}; 
use sha2::{Digest, Sha256};

pub const DEFAULT_TOKEN_SIZE: usize = 96; 


pub enum CodeVerifierError {
        InvalidSize,
}

pub fn gen_code_verifier(n: Option<usize>) -> Result<String, CodeVerifierError> {
    let size = n.unwrap_or_else(|| DEFAULT_TOKEN_SIZE);
    if size < 48 || size > 128 {
        return Err(CodeVerifierError::InvalidSize);
    }
    Ok(token_urlsafe(size))
}

pub fn gen_code_challenge(code_verifier: &str) -> String {
    return urlsafe_b64encode(&Sha256::digest(code_verifier));
}

#[derive(Debug, Clone)]
struct PKCE {
    code_verifier: String,
    code_challenge: String,
}

impl PKCE {
    pub fn new() -> Self {
        let code_verifier = token_urlsafe(96);
        let code_challenge = gen_code_challenge(&code_verifier);

        Self {
            code_verifier,
            code_challenge,
        }
    }

    fn get_code_verifier(&self) -> String {
        self.code_verifier.clone()
    }

    fn  get_code_challenge(&self) -> String {
        self.code_challenge.clone()
    }
    fn  get_method(&self) -> String {
        "S256".to_string()
    }
}



fn main() {
    let pkce: PKCE = PKCE::new();

    println!("Value: {}",pkce.get_code_challenge());
    println!("Value: {}",pkce.get_code_verifier());
    println!("Value: {}",pkce.get_method());


}