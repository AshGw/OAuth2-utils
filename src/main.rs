mod utils;
mod consts;
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};

use utils::{urlsafe_chars,token_urlsafe}; 
use consts::base64_encode; 

fn gen_oauth_params() -> (String, String, String, &'static str) {
    let state: String = OsRng.next_u32().to_string();
    let mut code_verifier: [u8; 96] = [0u8; 96];
    OsRng.fill_bytes(&mut code_verifier);

    let code_challenge = base64_encode(&Sha256::digest(&code_verifier));
    let code_challenge_method = "S256";

    (state, base64_encode(code_verifier), code_challenge, code_challenge_method)
}

fn gen_csrf_token() -> String {
    let csrf_token: String = OsRng.next_u32().to_string();
    csrf_token
}

fn main() {

    let (state, code_verifier, code_challenge, code_challenge_method) = gen_oauth_params();
    println!("State: {}", state);
    println!("Code Verifier: {}", code_verifier);
    println!("Code Challenge: {}", code_challenge);
    println!("Code Challenge Method: {}", code_challenge_method);

    let csrf_token = gen_csrf_token();
    println!("CSRF Token: {}", csrf_token);
    println!("urlsafe chars {}",urlsafe_chars()); 
    println!("Token_rul safex: {}",token_urlsafe(Some(96)));


}
