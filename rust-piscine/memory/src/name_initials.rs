/*
Create a function named initials which takes a vector of string literals and returns its initials in the format "J. P." (with a space between the initials) as a vector of String.
To make it easier, we will make a function call name_initials that takes a string literal and returns its initials in the same format first.
*/

pub fn name_initials(name: &str) -> String {
    let mut initials = String::new();
    let mut chars = name.chars();
    initials.push(chars.next().unwrap());
    initials.push_str(". ");
    for c in chars {
        if c.is_ascii_uppercase() {
            initials.push(c);
            initials.push('.');
            break;
        }
    }

    initials
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let initials = names.iter().map(|name| name_initials(name)).collect();
    initials
}
