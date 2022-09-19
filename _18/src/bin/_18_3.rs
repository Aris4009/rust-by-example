fn main() {
    let twenty = multiply("10", "20");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_nubmer = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_nubmer * second_number
}
