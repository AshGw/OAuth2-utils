use oauth2_utils::urlsafe::b64::{urlsafe_b64decode,urlsafe_b64encode};
use oauth2_utils::errors::B64Error;
use std::borrow::Cow;

fn main() {
    let a: String = String::from("some value"); 
    let encoded: String = urlsafe_b64encode(a); 
    println!("{}",encoded);
    let decoded: Result<Cow<'_, str>, B64Error> = urlsafe_b64decode(&encoded); 
    println!("{:?}",decoded);
}

