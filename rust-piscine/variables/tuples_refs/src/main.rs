/*
Define a tuple struct to represent Student. Each is identified by an ID of type u32, their first name and their last name
The define three functions to return the id, first name and last name
*/
mod student;
use student::{first_name, id, last_name, Student};
mod test;
fn main() {
    let mut student = Student(1, "Jordan".to_string(), "Peterson".to_string());
    println!("Student: {:?}", student);
    println!("Student ID: {}", id(&mut student));
    println!("Student first name: {}", first_name(&student));
    println!("Student last name: {}", last_name(&student));

    let mut student = Student(2, "Andrew".to_string(), "Huberman".to_string());
    println!("Student: {:?}", student);
    println!("Student ID: {}", id(&mut student));
    println!("Student first name: {}", first_name(&student));
    println!("Student last name: {}", last_name(&student));
}
