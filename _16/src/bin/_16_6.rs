fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2),3);
}

fn make_adder_function(y:i32)-> impl Fn(i32)-> i32{
   move |x:i32|{x+y}
}