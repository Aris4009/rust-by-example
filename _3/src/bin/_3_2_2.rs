#![allow(dead_code)]

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    println!("One is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x0ff00,
    Blue = 0x0000ff,
}
