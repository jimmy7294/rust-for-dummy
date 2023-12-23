#[cfg(test)]
mod tests {
    use crate::division_and_remainder::divide;
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 3), (3, 1));
        assert_eq!(divide(100, 30), (3, 10));
    }
}
