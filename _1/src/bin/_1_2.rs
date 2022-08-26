use std::fmt::Display;

fn main() {
    println!("{} days", 31);
    println!("{} days", 32i64);

    println!("{0},this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the layz dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);

    #[allow(dead_code)]
    struct Structure(i32);
    impl Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    eprintln!("io::stderr");

    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("This struct `{}` can print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);
}
