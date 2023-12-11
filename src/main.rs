use base64::{Engine, engine::{self, general_purpose}, alphabet};
use rand::{Rng,thread_rng};
use sha2::{Digest, Sha256};

pub const DEFAULT_TOKEN_SIZE: usize = 96; 


fn _urlsafe_chars() -> String {
    (b'A'..=b'Z')
    .map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_~".chars())
    .collect()
}

pub fn token_urlsafe(n: usize) -> String {    
    (0..n)
    .map(|_| {
        _urlsafe_chars()
        .chars()
        .nth(thread_rng().gen_range(0.._urlsafe_chars().len()))
        .unwrap()
    })
    .collect()
}



pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    B64.encode(token)
}

const B64: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);


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