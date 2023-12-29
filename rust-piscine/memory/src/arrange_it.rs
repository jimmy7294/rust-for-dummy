/*
create a function that takes a string literal, sorts the words and returns it. Each word will
contain the number that indicates the position of the word in the string.
*/
// we will construct a new string from the given string, the new string will be sorted by the number
// then we will contruct another string from the new string, but this time we will remove the number from the string
// by iterating over each word and removing the number from it and pushing it to the new string.
/* pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by(|a, b| {
        let a_num: u32 = a
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let b_num: u32 = b
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        a_num.cmp(&b_num)
    });

    let words = words
        .iter()
        .map(|w| remove_number(*w))
        .collect::<Vec<String>>();
    words.join(" ")
}

fn remove_number(word: &str) -> String {
    word.chars().filter(|c| !c.is_numeric()).collect()
} */

pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by(|a, b| {
        let a_num: u32 = a
            .chars()
            .filter_map(|c| c.to_digit(10)) // filter_map is like filter but it also maps the value
            .fold(0, |a, b| a * 10 + b); // fold will return the number that is mode from the digits from the word, it means that it can return 2 or more digits number
                                         // println!(" a: {:?}", a);
                                         // println!("a.chars(): {:?}", a.chars());
                                         // println!(
                                         //     "a.chars().filter_map(|c| c.to_digit(10)): {:?}",
                                         //     a.chars().filter_map(|c| c.to_digit(10))
                                         // );
                                         // println!("a_num: {:?}", a_num);
        let b_num: u32 = b
            .chars()
            .filter_map(|c| c.to_digit(10))
            .fold(0, |a, b| a * 10 + b);
        // println!("b_num: {:?}", b_num);
        a_num.cmp(&b_num)
    });

    let words = words
        .iter()
        .map(|w| remove_number(w))
        .collect::<Vec<String>>();
    words.join(" ")
}

fn remove_number(word: &str) -> String {
    let mut word = word.to_string();
    word.retain(|c| !c.is_numeric()); // retain only works on String and Vec, not on &str
    word
}
