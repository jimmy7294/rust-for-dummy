/*
Create 5 functions:
- sum: returns the sum between two values from 0 to 255
- diff: returns the difference between two values from -32768 to 32767
- pro: returns the product of the multiplication between two values from -128 to 127
- quo: returns the quotient of the division between two 32bit values
- rem: returns the remainder of the division between two 32bit values
*/

pub mod test;

pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

fn main() {
    println!("Hello, world!");
}
