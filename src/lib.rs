//! This crate provides utility functions for working with OAuth2:
//!
//! - PKCE
//! - URL-safe tokens
//! - URL-safe base64 encoding/decoding
//!
//! ## Installation
//!
//! ```shell
//! cargo add oauth2_utils
//! ```
//!
//! ## Usage
//!
//! To generate a PKCE pair with the corresponding method (SHA256 by default):
//!
//! ```rust
//! use oauth2_utils::pkce::PKCE;
//!
//! let pkce = PKCE::new();
//!
//! println!("PKCE Code Challenge: {}", pkce.code_challenge);
//! println!("PKCE Code Verifier: {}", pkce.code_verifier);
//! println!("PKCE Code method: {}", pkce.method);
//! ```
//! To generate a code verifier with a custom length:
//!
//! ```rust
//! use oauth2_utils::errors::CodeVerifierError;
//!
//! use oauth2_utils::pkce::gen::{gen_code_challenge, gen_code_verifier};
//!
//!
//! pub fn main() -> Result<(), CodeVerifierError> {
//! let code_verifier = gen_code_verifier(Some(128))?;
//! eprintln!("Code Verifier: {}", code_verifier);
//! let code_challenge = gen_code_challenge(&code_verifier);
//! eprintln!("Code Challenge: {}", code_challenge);
//! Ok(())
//! }
//! ```
//! To generate a URL-safe token for Nonce, State, etc..
//!
//! ```rust
//! use oauth2_utils::urlsafe::urlsafe_token;
//!
//! println!("URL-safe Token: {}", urlsafe_token(32))
//! ```
//!
//! For base64 encoding/decoding operations:
//! ```rust
//!
//! use oauth2_utils::errors::B64Error;
//! use oauth2_utils::urlsafe::b64::{urlsafe_b64decode, urlsafe_b64encode};
//!
//! pub fn main() -> Result<(), B64Error> {
//! let val = String::from("some value");
//! let encoded = urlsafe_b64encode(val);
//! println!("{}", encoded);
//! let decoded = urlsafe_b64decode(encoded)?;
//! println!("{}", decoded);
//! Ok(())
//! }
//!```
//!
pub mod consts;
pub mod errors;
pub mod pkce;
pub mod urlsafe;
