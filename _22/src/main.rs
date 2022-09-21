use std::slice;
fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }

    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    unsafe {
        let my_slice = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
