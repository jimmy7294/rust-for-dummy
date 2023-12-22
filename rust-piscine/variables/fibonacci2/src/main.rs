// Create a function called fibonacci that takes a number and return the fibonacci number of that position.
mod fibonacci2;
mod test;
use fibonacci2::fibonacci;
fn main() {
    println!("Hello, world!");
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("fibonacci(20) = {}", fibonacci(20));
    println!("fibonacci(3) = {}", fibonacci(3));
}
