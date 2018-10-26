use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({} {})", self.0, self.1);
        write!(f, "({} {})", self.2, self.3)
    }
}


fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let pair = (1, true);
    let revered = reverse(pair);

    println!("{:?} - {:?}", pair, revered);

    println!("Single element tuple {:?}", (123u32,));

    let tuple = (1, "foo", true);
    let (a, b, _) = tuple;
    println!("{a} {b}", a=a, b=b);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix\n{}", matrix);
    println!("Transposed\n{}", transpose(matrix));
}