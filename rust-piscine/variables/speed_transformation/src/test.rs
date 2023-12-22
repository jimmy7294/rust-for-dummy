#[cfg(test)]

mod tests {
    use crate::speed_transformation::*;
    #[test]
    fn kmh_ms() {
        assert_eq!(kmh_to_ms(100.0), 27.77777777777778);
        assert_eq!(kmh_to_ms(10.0), 2.7777777777777777);
        assert_eq!(kmh_to_ms(90.0), 25.0);
        assert_eq!(kmh_to_ms(50.0), 13.88888888888889);
    }
}
