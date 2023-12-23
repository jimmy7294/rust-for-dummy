#[cfg(test)]
mod tests {
    use crate::matrix_transposition::*;
    #[test]
    fn test_matrix_tranpose() {
        let matrix = Matrix((1, 2), (3, 4));
        assert_eq!(transpose(matrix), Matrix((1, 3), (2, 4)));
    }
}
