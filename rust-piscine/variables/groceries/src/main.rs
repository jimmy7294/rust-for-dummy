// create a function named insert that appends a new element at the end of the Vec
// create a function named at_index that returns the element at the given index
mod functions;
mod test;
use functions::{at_index, insert};
fn main() {
    println!("Hello, world!");
    let mut groceries = vec![
        "milk".to_string(),
        "eggs".to_string(),
        "yoghurt".to_string(),
    ];
    insert(&mut groceries, "bananas".to_string());
    println!("groceries: {:?}", groceries);
    insert(&mut groceries, "oats".to_string());
    println!("groceries: {:?}", groceries);
    insert(&mut groceries, "steak".to_string());
    println!("groceries: {:?}", groceries);
    insert(&mut groceries, "pulled pork".to_string());
    println!("groceries: {:?}", groceries);

    let item = at_index(&mut groceries, 0);
    println!("item: {}", item);
    let item = at_index(&mut groceries, 1);
    println!("item: {}", item);
    let item = at_index(&mut groceries, 2);
    println!("item: {}", item);
    let item = at_index(&mut groceries, 199);
    println!("item: {}", item);
    let item = at_index(&mut groceries, 299);
    println!("item: {}", item);
}
