use oauth2_utils::pkce::PKCE;

fn main() {
    // Create a new PKCE instance and print its code challenge and code verifier
    let pkce = PKCE::new();
    println!("PKCE Code Challenge: {}", pkce.code_challenge);
    println!("PKCE Code Verifier: {}", pkce.code_verifier);
    println!("PKCE Code Verifier: {}", pkce.method);

}