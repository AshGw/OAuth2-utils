use rand::{Rng,thread_rng};

const DEFAULT_TOKEN_SIZE: usize = 96; 

pub fn token_urlsafe(n: Option<usize>) -> String {
    let urlsafe_chars: String = _urlsafe_chars();
    
    (0..n.unwrap_or(DEFAULT_TOKEN_SIZE))
        .map(|_| {
            urlsafe_chars
            .chars()
            .nth(thread_rng().gen_range(0..urlsafe_chars.len()))
            .unwrap()
        })
        .collect()
}

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


