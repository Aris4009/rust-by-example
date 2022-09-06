fn main() {
    let a = some_fn();
    println!("This function returns and you can see this line.");

    let x = foo();
    println!("You will never see this line!");
}

fn foo() -> ! {
    panic!("This call never returns");
}

fn some_fn() {
    ()
}
