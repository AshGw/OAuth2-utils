use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};


fn from_urlsafe_chars() -> String {
    let uppercase: std::iter::Map<std::ops::RangeInclusive<u8>, fn(u8) -> char> = (b'A'..=b'Z').map(char::from);
    let lowercase: std::iter::Map<std::ops::RangeInclusive<u8>, fn(u8) -> char> = (b'a'..=b'z').map(char::from);
    let digits: std::iter::Map<std::ops::RangeInclusive<u8>, fn(u8) -> char> = (b'0'..=b'9').map(char::from);

    let urlsafe_chars: String = uppercase.chain(lowercase).chain(digits)
        .chain("-_".chars())
        .collect();

    urlsafe_chars
}

fn token_urlsafex(n: Option<usize>) -> String {
    let n_default: usize = n.unwrap_or(64);
    let urlsafe_chars = from_urlsafe_chars();

    let mut rng = rand::thread_rng();
    let token: String = (0..n_default)
        .map(|_| {
            let idx = rng.gen_range(0..urlsafe_chars.len());
            urlsafe_chars.chars().nth(idx).unwrap()
        })
        .collect();

    token
}
fn token_urlsafe(n: Option<usize>) -> String {
    let n_default: usize = n.unwrap_or(96);
    let rng: rand::prelude::ThreadRng = thread_rng();
    let token: String = rng
        .sample_iter(&Alphanumeric)
        .take(n_default)
        .map(char::from)
        .collect();
    token.chars().collect()
}

const BASE_64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);


fn gen_oauth_params() -> (String, String, String, &'static str) {
    let state: String = OsRng.next_u32().to_string();
    let mut code_verifier: [u8; 96] = [0u8; 96];
    OsRng.fill_bytes(&mut code_verifier);

    let code_challenge = BASE_64.encode(&Sha256::digest(&code_verifier));
    let code_challenge_method = "S256";

    (state, BASE_64.encode(code_verifier), code_challenge, code_challenge_method)
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
    println!("The generated csrf token: {}",token_urlsafe(Some(64)));
    println!("{}",from_urlsafe_chars()); 

}
