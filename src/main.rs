use oauth2_utils::pkce::gen::{gen_code_verifier,gen_code_challenge};
use oauth2_utils::urlsafe::{urlsafe_token};
use oauth2_utils::pkce::PKCE; 


// run a testing module for all of these 

fn main() {
    let cv =  gen_code_verifier(Some((80))); 
    println!("{}",cv) ; 
    println!("{:?}",gen_code_challenge(&"rtrtg"));
    println!("{}",urlsafe_token(45));
    let pkce = PKCE::new();
    println!("{}",pkce.code_challenge);
    println!("{}",pkce.code_verifier);
}
