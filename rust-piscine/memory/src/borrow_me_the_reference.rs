/*
Create 2 functions:
- delete_and_backspace: receives the borrowed string and processes it. '-' represents the backspace and '+' represents the delete key,
so that "helll-o" and "he+lllo" are both converted to "hello". The function should modify the string in place, the '-' and '+' are consumed during the process.
- do_operations: which borrows a Vector of string literals representing simple addition and subtraction operations. The function should
replace the operations with the results i.e. ["2+2", "3+2", "10-3", "5+5"] should be converted to ["4", "5", "7", "10"].
*/

use std::char;

pub fn delete_and_backspace(s: &mut String) {
    //"bpp--o+er+++sskroi-++lcw"
    let mut i = 0;
    while i < s.len() {
        if s.chars().nth(i).unwrap() == '-' {
            s.remove(i); // remove the '-'
            if i > 0 {
                // if not the first fchar
                s.remove(i - 1); // remove the char before the '-'
                i -= 1; // go back one char because we removed one char from the string (the char before the '-')
            }
        } else {
            i += 1;
        }
    }
    let check_string = s.replace('+', "");
    if check_string.len() == 0 {
        *s = "".to_string();
        return;
    }

    // "bo+er+++sskro++lcw"
    let mut i = s.len() - 1;
    while i > 0 {
        if s.chars().nth(i).unwrap() == '+' {
            if i + 1 < s.len() {
                s.remove(i); // remove the '+'
                s.remove(i); // remove the char after the '+'
            }
        } else {
            i -= 1;
            // This will handle the case where the first char is a '+'
            if i == 0 && s.chars().nth(i).unwrap() == '+' {
                s.remove(i);
                s.remove(i);
            }
        }
    }
    // "borrow"
}

// ["2+2", "3+2", "10-3", "5+5"] -> ["4", "5", "7", "10"]
pub fn do_operations(s: &mut Vec<String>) {
    /*
    - create a helper function that takes a string literal of form "x+y" or "x-y" and returns the result of the operation as a string
    - iterate over the vector and replace each string literal with the result of the operation
     */
    for i in 0..s.len() {
        s[i] = parse_string(s[i].clone());
    }
}

// parse and evaluate simple arimethic expressions represented as strings
fn parse_string(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut op = ' ';
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in chars {
        match c {
            '+' | '-' | '*' | '/' => {
                op = c;
                numbers.push(current_number.parse::<i32>().unwrap());
                current_number = String::new();
            }
            _ => current_number.push(c),
        }
    }
    current_number.parse::<i32>().unwrap();
    numbers.push(current_number.parse::<i32>().unwrap());

    let a = numbers[0];
    let b = numbers[1];

    match op {
        '+' => (a + b).to_string(),
        '-' => (a - b).to_string(),
        '*' => (a * b).to_string(),
        '/' => (a / b).to_string(),
        _ => panic!("Can't do the expression"),
    }
}

// fn delete_and_backspace_v2(s: &mut String)

/* pub fn delete_and_backspace_v2(s: &mut String) {
    // stpp--+k++eqring
    // stp__-__+__qring
    // stp-+qring
    // st____ring
    // string
    // first if the string only contains '+' or '-' return an empty string
    if s.replace('+', "").replace('-', "").len() == 0 {
        *s = "".to_string();
        return;
    }
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut i = 1;
    while i < chars.len() {
        // create an new string with the chars different from ' '
        let res = chars.iter().filter(|c| **c != ' ').collect::<String>();
        println!("res: {}", res);
        if !res.contains('-') && !res.contains('+') {
            *s = res;
            return;
        }
        // stpp--+k++eqring
        // stp__-__+__qring
        if i - 1 > 0 && chars[i] == '-' && chars[i - 1].is_alphabetic() {
            chars[i] = ' ';
            chars[i - 1] = ' ';
            println!("chars-: {:?}", chars);
        }
        if i + 1 < chars.len() && chars[i] == '+' && chars[i + 1].is_alphabetic() {
            chars[i] = ' ';
            chars[i + 1] = ' ';
            println!("chars+: {:?}", chars);
        }
        i += 1;
        println!("chars after: {:?}", chars);
    }
}
*/
