use super::error::CodeVerifierError; 
use sha2::{Digest, Sha256};
use crate::urlsafe::
{
    urlsafe_token,
    urlsafe_b64encode,
};

pub fn gen_code_verifier(n: Option<usize>) -> Result<String, CodeVerifierError> {
    let size: usize = n.unwrap_or_else(|| 96);
    if size < 48 || size > 128 {
        return Err(CodeVerifierError::InvalidSize);
    }
    Ok(urlsafe_token(size))
}

pub fn gen_code_challenge(code_verifier: &str) -> String {
    return urlsafe_b64encode(&Sha256::digest(code_verifier));
}
