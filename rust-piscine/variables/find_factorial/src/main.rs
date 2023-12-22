// create a function that returns factorial of a number
mod find_factorial;
use find_factorial::factorial;
mod test;
fn main() {
    println!("Hello, world!");
    println!("Factorial of 5 is {}", factorial(5));
    println!("Factorial of 7 is {}", factorial(7));
    println!("Factorial of 10 is {}", factorial(10));
    println!("Factorial of 1 is {}", factorial(1));
}
