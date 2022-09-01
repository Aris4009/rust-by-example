fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    let c = 'a';

    let b = true;

    println_size_of_val("x", &x);
    println_size_of_val("y", &y);
    println_size_of_val("z", &z);
    println_size_of_val("i", &i);
    println_size_of_val("f", &f);
    println_size_of_val("c", &c);
    println_size_of_val("b", &b);
}

fn println_size_of_val<T>(name: &str, x: &T) {
    println!("size of `{}` in bytes: {}", name, std::mem::size_of_val(x))
}
