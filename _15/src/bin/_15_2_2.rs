use std::fmt::Display;

fn main() {
    struct Person {
        name: String,
        age: u8,
    }
    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} is {} years old", self.name, self.age)
        }
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    let Person { name, ref age } = person;
    println!("name: {}", name);
    println!("age: {}", age);

    // println!("The person struct is {}", person);
    println!("The person's age from person struct is {}", person.age);
}
