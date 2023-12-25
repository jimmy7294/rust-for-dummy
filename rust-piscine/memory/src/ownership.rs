// Create a function called first_subword that takes ownership of a string and returns the first subword of the string. It should work with camelCase, PascalCase, snake_case

pub fn first_subword(s: String) -> String {
    let mut subword = String::new();
    let mut chars = s.chars();
    let mut c = chars.next().unwrap();

    if c.is_ascii_uppercase() {
        subword.push(c);

        c = match chars.next() {
            Some(c) => c,
            None => return subword,
        }
    }

    while c.is_ascii_lowercase() {
        subword.push(c);

        c = match chars.next() {
            Some(c) => c,
            None => break,
        }
    }
    subword
}
