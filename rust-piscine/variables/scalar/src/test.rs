#[cfg(test)]
mod tests {
    use crate::{diff, pro, quo, rem, sum};

    #[test]
    #[should_panic]
    fn test_panic_sum() {
        sum(25, 255);
    }

    #[test]
    #[should_panic]
    fn test_panic_diff() {
        diff(-32768, 32767);
    }

    #[test]
    #[should_panic]
    fn test_panic_pro() {
        pro(-128, 127);
    }

    #[test]
    fn pass() {
        assert_eq!(sum(1, 2), 3);
        assert_eq!(diff(1, 2), -1);
        assert_eq!(pro(1, 2), 2);
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(rem(22.0, 2.0), 0.0);
    }
}
