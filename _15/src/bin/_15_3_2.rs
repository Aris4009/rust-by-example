fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;
    // 数据可以通过引用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // let y = &point.y;
    // println!("Point Z coordinate is {}", point.z);
    // println!(
    //     "Point has coordinates: ({}, {}, {})",
    //     borrowed_point.x, another_borrow.y, point.z
    // );

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let _y = &point.y;
    println!("Point Z coordinate is {}", point.z);

    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
