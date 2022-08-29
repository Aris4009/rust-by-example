use std::fmt::Display;

fn main() {
    //这个结构体不能打印,因为没有实现fmt:Display
    #[allow(dead_code)]
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {0:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        _name: &'a str,
        _age: u8,
    }
    impl<'a> Display for Person<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "The person is {}, he is {}", self._name, self._age)
        }
    }

    let peter = Person {
        _name: "Peter",
        _age: 27,
    };

    println!("{:#?}", peter);
    println!("{}", peter);
}
