#[cfg(test)]
mod tests {
    use crate::find_factorial::factorial;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(87178291200, factorial(14));
        assert_eq!(6402373705728000, factorial(18));
        assert_eq!(121645100408832000, factorial(19));
        assert_eq!(2432902008176640000, factorial(20));
    }
}
