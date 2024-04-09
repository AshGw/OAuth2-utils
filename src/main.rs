use oauth2_utils::errors::CodeVerifierError;
use oauth2_utils::pkce::gen::{gen_code_challenge, gen_code_verifier};

type Res<T> = Result<T, CodeVerifierError>;

fn main() -> Res<()> {
    let custom_code_verifier = gen_code_verifier(Some(128))?;
    eprintln!("Custom Code Verifier: {}", custom_code_verifier);
    let custom_code_challenge = gen_code_challenge(&custom_code_verifier);
    eprintln!("Custom Code Challenge: {:?}", custom_code_challenge);
    Ok(())
}
