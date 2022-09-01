use std::{fmt::Display, num::ParseIntError, str::FromStr};

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let circle = Circle::from_str("999");
    println!("{}", circle.unwrap().to_string());

    let circle = Circle::from_str("s");
    assert_eq!(circle.is_err(), true);
    circle.expect("invalid params");
}

struct Circle {
    radius: i32,
}
impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(i) => Ok(Circle { radius: i }),
            Err(e) => Err(e),
        }
    }
}

// impl ToString for Circle {
//     fn to_string(&self) -> String {
//         format!("xxxxx{}", self.radius)
//     }
// }
