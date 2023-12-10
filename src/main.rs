mod utils;
mod base_64;
use sha2::{Digest, Sha256};

use utils::{urlsafe_chars,token_urlsafe}; 
use base_64::base64_encode; 

fn gen_oauth_params() -> (String, String, String, String) {
    let state: String = token_urlsafe(Some(96));
    let code_verifier: String =  token_urlsafe(Some(96));
    let code_challenge_method: String = "S256".to_string();
    let code_challenge: String = base64_encode(&Sha256::digest(&code_verifier));

    (state, code_verifier, code_challenge, code_challenge_method)
}


fn main() {
    let (
        state,
        code_verifier,
        code_challenge,
        code_challenge_method
        ) = gen_oauth_params();
    println!("State: {}", state);
    println!("Code Verifier: {}", code_verifier);
    println!("Code Challenge: {}", code_challenge);
    println!("Code Challenge Method: {}", code_challenge_method);

    println!("urlsafe chars {}",urlsafe_chars()); 
    println!("Token_rul safe: {}",token_urlsafe(Some(96)));


}
