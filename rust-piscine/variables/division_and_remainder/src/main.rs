// Create a function call divide that takes two i32 arguments and returns a tuples of the division and remainder of the two arguments.
mod division_and_remainder;
use division_and_remainder::divide;
mod test;
fn main() {
    let (div, rem) = divide(10, 3);
    println!("div: {}, rem: {}", div, rem);

    let (div, rem) = divide(199, 22);
    println!("div: {}, rem: {}", div, rem);
}
