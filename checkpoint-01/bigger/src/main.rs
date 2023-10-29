// Create the function `bigger` that gets the biggest positive number in the `HashMap`
use std::collections::HashMap;

fn bigger(hash: HashMap<&str, i32>) -> i32 {
    let mut biggest = std::i32::MIN;

    for (_, value) in hash {
        if value > biggest {
            biggest = value;
        }
    }
    biggest
}

fn main() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "The biggest of the elements in the Hashmap is {}",
        bigger(hash)
    );
}

#[test]
fn test_positive() {
    let mut f = HashMap::new();
    f.insert("Daniel", 12);
    f.insert("Ashley", 333);
    f.insert("Katie", 334);
    f.insert("Robert", 14);

    assert_eq!(334, bigger(f));
}

#[test]
fn test_long() {
    let mut f = HashMap::new();
    f.insert("Daniel", 41758712);
    f.insert("Ashley", 54551444);
    f.insert("Katie", 575556334);
    f.insert("Robert", 574148);

    assert_eq!(575556334, bigger(f));
}
