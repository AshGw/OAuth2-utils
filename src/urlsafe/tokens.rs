use rand::{Rng,thread_rng};

pub const DEFAULT_TOKEN_SIZE: usize = 96; 


fn _urlsafe_chars() -> String {
    (b'A'..=b'Z')
    .map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_~".chars())
    .collect()
}

pub fn token_urlsafe(n: usize) -> String {    
    (0..n)
    .map(|_| {
        _urlsafe_chars()
        .chars()
        .nth(thread_rng().gen_range(0.._urlsafe_chars().len()))
        .unwrap()
    })
    .collect()
}