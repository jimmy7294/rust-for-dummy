/*
Define a struct named Matrix as a tuple of 2 tuples. The nested tuple will contain 2 i32s.
Create a function named transpose that calculates the transposition of a 2x2 matrix.
The transposition of a matrix is a new matrix whose rows are the columns of the original.
Matrix must implement Debug, PartialEq, and Eq. You can you derive
*/
/*
EXPLANATION
Traits: Debug, PartialEq, Eq. THe Debug trait allows instances of Matrix to be formatted for output using the {:?} or {:#?} format specifier.
The PartialEq and Eq traits allow instances of Matrix to be compared for equality using the == or != operators.
The derive attribute can be used to automatically implement these traits for Matrix
*/
mod matrix_transposition;
mod test;
use matrix_transposition::*;
fn main() {
    let matrix = Matrix((1, 2), (3, 4));
    println!("Original matrix: {:?}", matrix);
    println!("Transposed matrix: {:?}", transpose(matrix));

    let matrix = Matrix((2, 4), (6, 9));
    println!("Original matrix: {:?}", matrix);
    println!("Transposed matrix: {:?}", transpose(matrix));
}
