use std::{thread, vec};

static NTHREADS: i32 = 10;

fn main() {
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }))
    }

    for child in children {
        // 等待线程结束。返回一个结果。
        let _ = child.join();
    }
}
