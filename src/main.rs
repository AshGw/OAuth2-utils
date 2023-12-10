mod utils;
mod base_64;
use sha2::{Digest, Sha256};

use utils::{token_urlsafe}; 
use base_64::urlsafe_b64encode; 



fn gen_oauth_params() -> (String, String, String, String) {
    let state: String = token_urlsafe(Some(96));
    let code_verifier: String =  token_urlsafe(Some(96));
    let code_challenge_method: String = "S256".to_string();
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let x = hasher.finalize();
    println!("{:?}",x);
    let code_challenge: String = urlsafe_b64encode(x);

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
    println!("Token_rul safe: {}",token_urlsafe(Some(96)));


}
