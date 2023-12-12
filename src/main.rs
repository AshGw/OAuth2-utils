use auth2_tokens::gen::{gen_code_verifier,gen_code_challenge, CodeVerifierError};
use auth2_tokens::urlsafe::{urlsafe_token}; 

fn main() {
    let cv: Result<String, CodeVerifierError> =  gen_code_verifier(Some(255)); 
    println!("{:?}", cv);
    println!("{:?}",gen_code_challenge(&"rtrtg"));
    println!("{}",urlsafe_token(45));
}