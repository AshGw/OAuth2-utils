use rand::{Rng,thread_rng};

pub fn urlsafe_token(n: usize) -> String {    
    let chars: String = urlsafe_chars(); 
    (0..n)
    .map(|_| {
        chars
        .chars()
        .nth(thread_rng().gen_range(0..chars.len()))
        .unwrap()
    })
    .collect()
}

fn urlsafe_chars() -> String {
    (b'A'..=b'Z')
    .map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_~".chars())
    .collect()
}

