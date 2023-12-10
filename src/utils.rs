use rand::{Rng};

pub fn urlsafe_chars() -> String {
    (b'A'..=b'Z').map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_".chars())
    .collect()
}


pub fn token_urlsafe(n: Option<usize>) -> String {
    let urlsafe_chars: String = urlsafe_chars();
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    
    (0..n.unwrap_or(64))
        .map(|_| {
            let idx: usize = rng.gen_range(0..urlsafe_chars.len());
            urlsafe_chars.chars().nth(idx).unwrap()
        })
        .collect()
}