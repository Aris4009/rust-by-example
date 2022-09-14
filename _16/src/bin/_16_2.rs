fn main() {
    let random_number = 0.7;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen a {}, and it says {}",
        animal.name(),
        animal.noise()
    );
}
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
    fn name(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }

    fn name(&self) -> &'static str {
        "Sheep"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }

    fn name(&self) -> &'static str {
        "Cow"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
