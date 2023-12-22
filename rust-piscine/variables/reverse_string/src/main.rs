// create a function named rev_str that takes a &str argument and returns a String with letters reversed
mod reverse_string;
use reverse_string::rev_str;
mod test;

fn main() {
    println!("Hello, world!");
    let s = "run";
    let rev = rev_str(s);
    println!("{} becomes {}", s, rev);
    let s = "rust";
    println!("{} becomes {}", s, rev_str(s));
    let s = "main.rs";
    println!("{} becomes {}", s, rev_str(s));
}
