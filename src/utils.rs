mod consts; 
use rand::{Rng,thread_rng};
use sha2::{Digest, Sha256};
use consts::DEFAULT_TOKEN_SIZE as usize;


fn _urlsafe_chars() -> String {
    (b'A'..=b'Z')
    .map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_".chars())
    .collect()
}

pub fn token_urlsafe(n: usize) -> String {    
    (0..n)
    .map(|_| {
        urlsafe_chars
        .chars()
        .nth(thread_rng().gen_range(0..urlsafe_chars.len()))
        .unwrap()
    })
    .collect()
}


