#[allow(path_statements)]
#[allow(unused_must_use)]
fn main() {
    let x = 5;
    x;
    x + 1;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + 1
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
