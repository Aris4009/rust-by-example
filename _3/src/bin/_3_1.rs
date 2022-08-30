use std::fmt::Display;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

//单元结构体
struct Unit;

//元组结构体
struct Pair(i32, f32);

//带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{}", peter);
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({} {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({} {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edgt,
    } = point;

    let _retangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edgt,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let r = Rectangle {
        top_left: Point { x: 10f32, y: 5f32 },
        bottom_right: Point { x: 10f32, y: 5f32 },
    };
    println!("r's area is :{}", rect_area(&r));

    let r = square(&point, 10f32);
    println!("r's area is :{}", rect_area(&r));
}

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle {
        top_left,
        bottom_right: _,
    } = r;
    top_left.x * top_left.y
}

fn square(p: &Point, f: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x: p.x, y: p.y },
        bottom_right: Point { x: f, y: f },
    }
}
