fn main() {
    for e in [
        WebEvent::KeyPress('x'),
        WebEvent::Paste("my test".to_string()),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageLoad,
        WebEvent::PageUnload,
    ]
    .iter()
    {
        inspect(e);
    }

    let _x = Operations::Add;
    println!("{}", _x.run(100, 99));
}

#[allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("paged \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
