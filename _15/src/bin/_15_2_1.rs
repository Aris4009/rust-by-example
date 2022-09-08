fn main() {
    let immutable_box = Box::new(5i32);
    println!("immutable_box contains {}", immutable_box);

    // *immutable_box = 4;

    let mut mutable_box = immutable_box;
    *mutable_box = 4;
    println!("mutable_box contains {}", mutable_box);
}
