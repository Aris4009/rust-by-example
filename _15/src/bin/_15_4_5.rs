fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

#[derive(Debug)]
struct Borrowed<'a> {
    _x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { _x: &10 }
    }
}
