#[cxx::bridge(namespace = "com::bridge::demo")]
mod ffi {
    extern "Rust" {
        fn add(x: i32, y: i32) -> i32;
        fn sub(x: i32, y: i32) -> i32;
        fn mul(x: i32, y: i32) -> i32;
        fn div(x: i32, y: i32) -> i32;
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else {
        x / y
    }
}
