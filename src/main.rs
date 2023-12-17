use oauth2_utils::pkce::gen::{gen_code_verifier,gen_code_challenge};
use oauth2_utils::urlsafe::{urlsafe_token,b64::{urlsafe_b64decode,urlsafe_b64encode}};
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
    let a: String = String::from("heyy"); 
    let encoded: String = urlsafe_b64encode(a); 
    println!("{}",encoded);
    println!("{:?}",urlsafe_b64decode(&encoded));
}


