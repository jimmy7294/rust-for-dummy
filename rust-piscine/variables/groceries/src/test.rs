#[cfg(test)]
mod tests {
    use crate::functions::*;
    #[test]
    fn test_insertions() {
        let mut groceries = Vec::new();
        insert(&mut groceries, "chocolate".to_string());
        assert_eq!(groceries, vec!["chocolate".to_string()]);
        insert(&mut groceries, "orange".to_string());
        assert_eq!(groceries, vec!["chocolate", "orange"]);
    }

    #[test]
    fn test_index() {
        let mut groceries = vec![
            "beans".to_string(),
            "nuts".to_string(),
            "rice".to_string(),
            "spaghetti".to_string(),
        ];
        assert_eq!(at_index(&mut groceries, 0), "beans");
        assert_eq!(at_index(&mut groceries, 1), "nuts");
        assert_eq!(at_index(&mut groceries, 2), "rice");
        assert_eq!(at_index(&mut groceries, 3), "spaghetti");
    }
}
