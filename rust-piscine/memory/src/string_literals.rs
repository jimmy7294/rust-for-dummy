/*
Create 5 functions with the following signatures:
is_empty: returns true if the string is empty.
is_ascii: returns true if all of the characters are within the ASCII range.
contains: returns true if the string contains the given pattern.
split_at: returns a tuple with the string split at the given index.
find: returns the index of the first occurrence of the given character.
*/

pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

pub fn contains(s: &str, p: &str) -> bool {
    s.contains(p)
}

pub fn split_at(s: &str, i: usize) -> (&str, &str) {
    s.split_at(i)
}

pub fn find(s: &str, c: char) -> usize {
    s.find(c).unwrap_or(usize::MAX)
}
