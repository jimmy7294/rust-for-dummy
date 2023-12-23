// Create a function called 'to_url' that takes a string as parameter and returns a String where every white space is replaced by '%20'.

// pub fn to_url(s: &str) -> String {
//     let mut res = String::new();
//     for c in s.chars() {
//         if c == ' ' {
//             res.push_str("%20");
//         } else {
//             res.push(c);
//         }
//     }
//     res
// }

// or using 'replace' method of str, which replace all occurrences of a pattern with a replacement string.
pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}
