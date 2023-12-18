pub mod gen;
use crate::urlsafe::urlsafe_token;
use gen::gen_code_challenge;

#[derive(Debug, Clone)]
pub struct PKCE {
    /// The randomly generated code verifier for PKCE.
    pub code_verifier: String,
    /// The code challenge derived from the code verifier.
    pub code_challenge: String,
    /// The method used to generate the code challenge, defaults to "S256".
    pub method: String,
}

impl PKCE {
    /// Generates a new PKCE instance with a randomly generated code verifier,
    /// a derived code challenge, and the specified method (default is "S256").
    ///
    /// # Returns
    ///
    /// A new PKCE instance with the generated code verifier, code challenge, and method..
    pub fn new() -> Self {
        let code_verifier = urlsafe_token(96);
        let code_challenge = gen_code_challenge(&code_verifier);
        let method = "S256".to_string();

        Self {
            code_verifier,
            code_challenge,
            method,
        }
    }
}

impl Default for PKCE {
    fn default() -> Self {
        Self::new()
    }
}
