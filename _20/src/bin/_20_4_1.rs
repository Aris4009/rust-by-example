use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("Cargo.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}
