use std::fmt::Display;

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("=================");
    println!("{}", transpose(&matrix));
    println!("=================");
    println!("{}", matrix);
}

struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(m: &Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
