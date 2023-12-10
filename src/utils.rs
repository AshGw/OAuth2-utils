pub fn urlsafe_chars() -> String {
    (b'A'..=b'Z').map(char::from)
    .chain( (b'a'..=b'z')
    .map(char::from))
    .chain((b'0'..=b'9')
    .map(char::from))
    .chain("-_".chars())
    .collect()
}