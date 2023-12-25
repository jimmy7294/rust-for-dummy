mod borrow;
use borrow::*;
mod doubtful;
use doubtful::*;
mod test;
mod to_url;
use to_url::*;
mod string_literals;
use string_literals::*;
mod name_initials;
use name_initials::*;
mod ownership;
use ownership::*;
mod copy;
use copy::*;

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
    println!("\x1b[1;33mEnd of string_literals.rs\x1b[0m\n");

    // name_initials.rs
    println!("\x1b[1;32mname_initials.rs\x1b[0m");
    let names = vec!["Jordan Peterson", "Carl Jung", "Marcus Aurelius", "M. W."];
    for name in names.clone() {
        println!("{} -> {}", name, name_initials(name))
    }
    println!("{:?}", initials(names));
    // expectect output: ["J. P.", "C. J.", "M. A.", "M. W."]

    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
    println!("\x1b[1;32mEnd of name_initials.rs\x1b[0m\n");

    // ownership.rs
    println!("\x1b[1;31mownership.rs\x1b[0m");
    let s1 = String::from("helloWorld");
    let s2 = String::from("snake_case");
    let s3 = String::from("camelCase");
    let s4 = String::from("just");

    println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
    println!("first subword({}) = {}", s2.clone(), first_subword(s2));
    println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
    println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
    println!("\x1b[1;31mEnd of ownership.rs\x1b[0m\n");

    // copy.rs
    println!("\x1b[1;32mcopy.rs\x1b[0m");
    let a: i32 = 0;
    let b = String::from("1 2 3 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));

    println!("\x1b[1;32mEnd of copy.rs\x1b[0m\n");
}
