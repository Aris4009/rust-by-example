fn main() {
    let (four,nine) = (4,9);

    print_ref(&four, &nine);

    failed_borrow();
}

fn print_ref<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    // let y:&'a i32 = &_x;
}
