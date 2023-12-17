use oauth2_utils::pkce::gen::{gen_code_verifier,gen_code_challenge};
use oauth2_utils::pkce::CodeVerifierError; 
use oauth2_utils::urlsafe::{urlsafe_token};
use oauth2_utils::pkce::PKCE; 



// run a testing module for all of these 

fn main() {
    let cv: Result<String, CodeVerifierError> =  gen_code_verifier(Some(255)); 
    match cv {
        Ok(result) => println!("Success: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    println!("{:?}",gen_code_challenge(&"rtrtg"));
    println!("{}",urlsafe_token(45));
    let pkce = PKCE::new();
    println!("{}",pkce.code_challenge);
    println!("{}",pkce.code_verifier);
    println!("{}",pkce.method);
}
