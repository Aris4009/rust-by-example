#![allow(dead_code)]
#![allow(unused)]

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(1);
    println!("{:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
