# OAuth2 Utils 

## Overview

This library provides utility functions for working with OAuth2
- PKCE 
- URL-safe tokens eneration 

## Examples 

To generate a PKCE pair with the according method: 

````rust
use oauth2_utils::pkce::PKCE;

fn main() {
    // Create a new PKCE instance and print its code challenge and code verifier
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

## License 
