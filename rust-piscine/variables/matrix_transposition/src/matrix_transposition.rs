#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));
pub fn transpose(matrix: Matrix) -> Matrix {
    Matrix((matrix.0 .0, matrix.1 .0), (matrix.0 .1, matrix.1 .1))
}
