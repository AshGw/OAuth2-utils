## Overview

This crate provides utility functions for working with OAuth2
- PKCE 
- URL-safe tokens 
- URL-safe base64 encoding/decoding

## Installation 
Add this to your `Cargo.toml`
```toml
[dependencies]
oauth2_utils = "0.0.1"
```
## Examples 

To generate a PKCE pair with the corresponding method: 

````rust
use oauth2_utils::pkce::PKCE;

fn main() {
    let pkce = PKCE::new();
    println!("PKCE Code Challenge: {}", pkce.code_challenge);
    println!("PKCE Code Verifier: {}", pkce.code_verifier);
    println!("PKCE Code method: {}", pkce.method);

}
````
The result should look something like this:
````commandline
PKCE Code Challenge: WTht2od6WZKdSSTEgqyfNuSSn3ykru-YDpI8nta20hI
PKCE Code Verifier: bEV4fZdeQaZ~oZc85-2YfYSl6zvPMa1xJJYT7cMRxNZ9L17Kqp89sAEPZJwEetp35W3~wJYzvaH6c9ktiR1oAzIAdF5s_dC4
PKCE Code Verifier: S256
````
To generate a code verifier with a custom length
```rust
use oauth2_utils::pkce::gen::{gen_code_verifier, gen_code_challenge};
use oauth2_utils::urlsafe::{urlsafe_token, urlsafe_b64encode};
use sha2::{Digest, Sha256};

fn main() {
    let custom_code_verifier = gen_code_verifier(Some(128)); // defaults to 98 if None
    println!("Custom Code Verifier: {}", custom_code_verifier);
    // // //
    let custom_code_challenge = gen_code_challenge(&custom_code_verifier);
    println!("Custom Code Challenge: {:?}", custom_code_challenge);
}
```

To generate a URL-safe token for Nonce, State, etc..
````rust
use oauth2_utils::urlsafe::urlsafe_token;

fn main() {
    println!("URL-safe Token: {}", urlsafe_token(32)) // of length 32;
}
````
For base64 encoding/decoding operations
```rust
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
```


## License 
[GPL-3.0](https://github.com/AshGw/oauth2_utils/blob/main/LICENSE)