mod utils;
mod base_64;
use sha2::{Digest, Sha256};

use utils::{token_urlsafe}; 
use base_64::urlsafe_b64encode; 


pub fn gen_state(n: Option<usize>) -> String {
    return token_urlsafe(Some(n.unwrap_or(DEFAULT_TOKEN_SIZE)));
}



pub fn gen_code_verifier(n: Option<usize>) -> String {
    return token_urlsafe(Some(n.unwrap_or(DEFAULT_TOKEN_SIZE)));
}

pub fn gen_code_challenge(code_verifier: &str) -> String {
    return urlsafe_b64encode(&Sha256::digest(code_verifier));
}



pub fn gen_oauth_params() -> (String, String, String, String, String) {
    let state: String = gen_state();
    let code_verifier: String =  gen_code_verifier();
    let code_challenge_method: String = "S256".to_string();
    let code_challenge: String = gen_code_challenge(&code_verifier);

    (state, code_verifier, code_challenge, code_challenge_method)
}
