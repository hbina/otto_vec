#[macro_use]
extern crate otto_vec;

#[otto_vec]
pub fn test(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}
