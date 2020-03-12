extern "C" {
    fn sum(x: i32, y: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn add_one(x: i32, y: i32) -> i32 {
    unsafe { sum(x, y) + 1 }
}
