pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    let empty = input.trim().is_empty();
    if empty {
        return res;
    }
    for (i, c) in input.chars().enumerate() {
        if i == 0 && c.is_ascii_alphabetic() {
            res.push(c.to_ascii_uppercase());
        } else {
            res.push(c);
        }
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let empty = input.trim().is_empty();
    if empty {
        return res;
    }
    // method 1, using prev = ' ' to check if the previous char is a space
    // let mut prev = ' ';
    // for c in input.chars() {
    //     if prev == ' ' && c.is_ascii_alphabetic() {
    //         res.push(c.to_ascii_uppercase());
    //     } else {
    //         res.push(c);
    //     }
    //     prev = c;
    // }
    // method 2, using split_whitespace() to split the string into words
    // and then capitalize the first letter of each word
    for word in input.split_whitespace() {
        for (i, c) in word.chars().enumerate() {
            if i == 0 && c.is_ascii_alphabetic() {
                res.push(c.to_ascii_uppercase());
            } else {
                res.push(c);
            }
        }
        res.push(' ');
    }
    res.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    let empty = input.trim().is_empty();
    if empty {
        return res;
    }
    for c in input.chars() {
        match c {
            'a'..='z' => res.push(c.to_ascii_uppercase()),
            'A'..='Z' => res.push(c.to_ascii_lowercase()),
            _ => res.push(c),
        }
    }
    res
}

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving Abilene"));
    println!("{}", change_case("heLLo THere"));
}

#[test]
fn test_success() {
    assert_eq!(capitalize_first("hello"), "Hello");
    assert_eq!(capitalize_first("this is working"), "This is working");
}

#[test]
fn test_title_case() {
    assert_eq!(title_case("this is a little"), "This Is A Little");
    assert_eq!(title_case("hello my name is carl"), "Hello My Name Is Carl");
}

#[test]
fn test_change_case() {
    assert_eq!(change_case("PROgraming"), "proGRAMING");
    assert_eq!(change_case("heLLo THere"), "HEllO thERE");
}

#[test]
fn test_empty() {
    assert_eq!(capitalize_first(""), "");
    assert_eq!(title_case(""), "");
    assert_eq!(change_case(""), "");
}
