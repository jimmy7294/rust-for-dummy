#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &mut Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}

/*
In Rust, the &mut is used to create a mutable reference to a value. A mutable reference allows you to modify the value it points to.

fn main() {
    let mut x = 10;
    add_five(x);
    println!("x: {}", x);
}

fn add_five(mut num: i32) {
    num += 5;
}

The above code will print x: 10. This is because the add_five function takes ownership of the value passed to it. This means that the value is copied into the function and the original value is not modified.

----
fn main() {
    let mut x = 10;
    add_five(&mut x);
    println!("x: {}", x);
}

fn add_five(num: &mut i32) {
    *num += 5;
}

The above code will print x: 15. This is because the add_five function takes a mutable reference to the value passed to it. This means that the value is not copied into the function and the original value is modified.
Inside the add_five function, num is a mutable reference to an i32. The *num += 5; line dereferences num to get the i32 value it points to, and then adds 5 to its value. Because num is a mutable reference, this operation is allowed and it modifies the original x value in the main function.
*/
