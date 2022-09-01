#![allow(overflowing_literals)]

fn main() {
    let decimail = 65.4321_f32;

    let integer = decimail as u8;

    let character = integer as char;

    println!("Casting: {} -> {} ->{}", decimail, integer, character);
    print_split();

    println!("1000 as a u16 is: {}", 1000 as u16);
    print_split();

    process_casting_1();
    println!("1000 as a u8 is: {}", 1000 as u8);
    print_split();

    process_casting_2();
    println!("-1 as a u8 is: {}", -1i32 as u8);
    print_split();

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。
    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(
        " 128 as a i8 is : {0},{0:b},{1}",
        128 as i8,
        128 & 0b11111111
    );

    print_split();
    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn process_casting_1() {
    println!("1000的二进制是{:b}", 1000);
    println!(
        "从最低位开始,保留8位:{:b},{1:b}->{1},{2}",
        1000,
        0b11111111,
        1000 & 255
    );
}

fn process_casting_2() {
    //负数的二进制
    /*
      1.先取正数的二进制,-1取1的二进制位   0000 0000 0000 0000 0000 0000 0000 0001
      2.取该数的反码为                   1111 1111 1111 1111 1111 1111 1111 1110
      3.反码+1即为补码                   1111 1111 1111 1111 1111 1111 1111 1111
    */
    println!("-1的二进制是:{:b}", -1);
    println!(
        "从最低位开始,保留8位:{:b},{1:b}->{1},{2}",
        -1,
        0b11111111,
        -1 & 255
    );
}

fn print_split() {
    println!("===================================");
}
