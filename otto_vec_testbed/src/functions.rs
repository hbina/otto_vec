/*
#[allow(dead_code)]
#[otto_vec]
pub(crate) fn function_1(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
#[otto_vec]
pub(crate) fn function_2<X: Into<i32>, Y: Into<i32>>(a: X, b: Y) -> i32 {
    let result = 10;
    result * a.into() + b.into()
}

#[allow(dead_code)]
#[otto_vec]
pub(crate) fn function_3(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if let Some(a) = a {
        if let Some(b) = b {
            return Some(a + b);
        }
    }
    None
}
*/

pub struct Location {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
#[otto_vec]
fn fn_4((Location { x, .. }, Location { y, .. }): (Location, Location)) -> i64 {
    x * y
}
