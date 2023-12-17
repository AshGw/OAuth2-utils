# OAuth2 Utils 

## Overview

This crate provides utility functions for working with OAuth2
- PKCE 
- URL-safe tokens generation 
- URL-safe base64 encoding/decoding

## Examples 

To generate a PKCE pair with the according method: 

````rust
use oauth2_utils::pkce::PKCE;

fn main() {
    let pkce = PKCE::new();
    println!("PKCE Code Challenge: {}", pkce.code_challenge);
    println!("PKCE Code Verifier: {}", pkce.code_verifier);
    println!("PKCE Code Verifier: {}", pkce.method);

}
````
The result should look something like this:
````commandline
PKCE Code Challenge: WTht2od6WZKdSSTEgqyfNuSSn3ykru-YDpI8nta20hI
PKCE Code Verifier: bEV4fZdeQaZ~oZc85-2YfYSl6zvPMa1xJJYT7cMRxNZ9L17Kqp89sAEPZJwEetp35W3~wJYzvaH6c9ktiR1oAzIAdF5s_dC4
PKCE Code Verifier: S256
````
To accommodate a custom code verifier length you can use 
```rust
use oauth2_utils::pkce::gen::{gen_code_verifier, gen_code_challenge};
use oauth2_utils::urlsafe::{urlsafe_token, urlsafe_b64encode};
use sha2::{Digest, Sha256};

fn main() {
    // Generate a custom code verifier with a specified length (e.g., 128 characters)
    let custom_code_verifier = gen_code_verifier(Some(128)); // defaults to 98 if None
    println!("Custom Code Verifier: {}", custom_code_verifier);

    // Generate a custom code challenge from the custom code verifier
    let custom_code_challenge = gen_code_challenge(&custom_code_verifier);
    println!("Custom Code Challenge: {:?}", custom_code_challenge);
}
```


To generate URL-safe token to use for Nonce or State values you can use 
````rust
use oauth2_utils::urlsafe::urlsafe_token;

fn main() {
    println!("URL-safe Token: {}", urlsafe_token(45)) // of length 45;
}
````


## License 
[GPL-3.0](LICENSE)