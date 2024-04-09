use crate::consts::URL_SAFE_CHARS;
use rand::{thread_rng, Rng};

/// Generates a random string of length `n` from URL-safe characters.
pub fn urlsafe_token(n: usize) -> String {
    (0..n)
        .map(|_| {
            URL_SAFE_CHARS
                .chars()
                .nth(thread_rng().gen_range(0..URL_SAFE_CHARS.len()))
                .unwrap()
        })
        .collect()
}
