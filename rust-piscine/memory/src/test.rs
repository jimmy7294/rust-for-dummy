#[cfg(test)]
mod tests {
    use crate::borrow::*;
    use crate::doubtful::*;

    #[test]
    fn test_borrow() {
        assert_eq!(str_len("Compiling"), 9);
        assert_eq!(str_len(&"Solarion".to_string()), 8);
    }

    #[test]
    fn test_doubtful() {
        let mut s = String::from("Aurilius");
        doubtful(&mut s);
        assert_eq!(s, "Aurilius😀")
    }

    use crate::to_url::to_url;
    #[test]
    fn test_to_url() {
        let s = "The impediment of action advances action. What stands in the way becomes the way.";
        assert_eq!(to_url(s), "The%20impediment%20of%20action%20advances%20action.%20What%20stands%20in%20the%20way%20becomes%20the%20way.");
    }

    use crate::string_literals::*;
    #[test]
    fn test_string_literals() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_empty("Rust"), false);
        assert_eq!(
            is_ascii("You're not everything you could be. And you know it."),
            true
        );
        assert_eq!(is_ascii(" 😁"), false);
        assert_eq!(contains("Face the demands of life voluntarily. Respond to a challenge, instead of bracing for catastrophe.", "challenge"), true);
        assert_eq!(contains("Face the demands of life voluntarily. Respond to a challenge, instead of bracing for catastrophe.", "love"), false);
        assert_eq!(split_at("You can do an awful lot by writing down what happened to you and thinking it through", 10), ("You can do", " an awful lot by writing down what happened to you and thinking it through"));
        assert_eq!(find("You can do an awful lot by writing down what happened to you and thinking it through", 'w'), 15);
    }
}
