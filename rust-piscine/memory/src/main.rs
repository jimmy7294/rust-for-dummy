mod borrow;
use borrow::*;
mod doubtful;
use doubtful::*;
mod test;
mod to_url;
use to_url::*;
mod string_literals;
use string_literals::*;

fn main() {
    // borrow.rs
    println!("\x1b[1;34mborrow.rs\x1b[0m");
    let s = "hello";
    let s1 = "camelCase".to_string();
    let s2 = "snailCase".to_string();

    println!("\tstr_len(\"{}\") = {}", s, str_len(s));
    println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
    println!("str_len({}) = {}", s2, str_len(&s2));
    println!("\x1b[1;34mEnd of borrow.rs\x1b[0m\n");

    // doubtful.rs
    let mut s = String::from("Hellno");
    println!("\x1b[1;35mdoubtful.rs\x1b[0m");
    println!("Before changing the string: {}", s);
    doubtful(&mut s);
    println!("After changing the string: {}", s);
    println!("\x1b[1;35mEnd of doubtful.rs\x1b[0m\n");

    // to_url.rs
    let s = "If you want to improve, be content to be thought foolish and stupid.";
    println!("\x1b[1;36mto_url.rs\x1b[0m");
    println!("Before to_url: {}", s);
    println!("After to_url: {}", to_url(s));
    println!("\x1b[1;36mEnd of to_url.rs\x1b[0m\n");

    // string_literals.rs
    println!("\x1b[1;33mstring_literals.rs\x1b[0m");
    println!("{}", is_empty(""));
    println!("{}", is_ascii("Jordan Peterson"));
    println!("{}", is_ascii("😀"));
    println!(
        "{}",
        contains("There is a fine line between genius and insanity.", "fine")
    );
    println!("{:?} ", split_at("Order and Chaos", 3));
    println!("{:?}", split_at("Rust is fun", 6));
    println!(
        "{}",
        find(
            "Compare yourself to who you were yesterday, not to who someone else is today.",
            'y'
        )
    );
    println!("{}", find("Rust is tough", 'z'));
    println!("\x1b[1;33mEnd of string_literals.rs\x1b[0m\n")
}
