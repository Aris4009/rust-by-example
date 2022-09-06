fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i: i32| i + 1;
    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    let add = |i: &mut i32| *i += 1;

    let mut i = 1;
    let mut v = Vec::with_capacity(10);
    for _ in 1..=10 {
        v.push(i);
        add(&mut i);
    }
    println!("{:?}", v);
}
