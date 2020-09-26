#[otto_vec]
pub fn function_1(a: i32, b: i32) -> i32 {
    a + b
}

#[otto_vec]
pub fn function_2<X: Into<i32>, Y: Into<i32>>(a: X, b: Y) -> i32 {
    let result = 10;
    result * a.into() + b.into()
}

#[otto_vec]
pub fn function_3(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    if let Some(a) = a {
        if let Some(b) = b {
            return Some(a + b);
        }
    }
    return None;
}
