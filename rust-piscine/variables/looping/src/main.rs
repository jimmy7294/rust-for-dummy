use std::io;

const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
const ANSWER: &str = "The letter e";
fn main() {
    println!("Riddle: {}", RIDDLE);
    println!("Enter your answer: ");
    let mut tries = 0;
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == ANSWER || guess.trim() == "e" {
            println!("Correct!\nNumber of tries: {}", tries);
            break;
        } else {
            tries += 1;
        }
    }
}
