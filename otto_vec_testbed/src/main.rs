#[allow(dead_code)]
#[macro_use]
extern crate otto_vec;

#[otto_vec]
pub fn test(a: i32, b: i32) -> i32 {
    a + b
}

#[otto_vec]
pub fn test2<X: Into<i32>, Y: Into<i32>>(a: X, b: Y) -> i32 {
    let result = 10;
    a + b + result
}

fn main() {}
