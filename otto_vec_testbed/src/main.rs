#[allow(dead_code)]
#[macro_use]
extern crate otto_vec;

#[otto_vec]
pub fn function_1(a: i32, b: i32) -> i32 {
    a + b
}

#[otto_vec]
pub fn function_2<X: Into<i32>, Y: Into<i32>>(a: X, b: Y) -> i32 {
    let result = 10;
    result * a.into() + b.into()
}

/// This application exists solely for development purposes.
/// In particular, we can see what the macros expand into.
fn main() {}

#[test]
fn test_function_1() {
    let expected = vec![2, 5, 7, 12, 3];
    let vector_1 = vec![1, 3, 3, 7, 2];
    let vector_2 = vec![1, 2, 4, 5, 1];
    assert_eq!(expected, function_1_vec(vector_1, vector_2));
}

#[test]
fn test() {
    #[derive(Debug, Eq, PartialEq)]
    struct S {
        i: i32,
    }
    impl S {
        pub fn new(i: i32) -> Self {
            S { i }
        }
    }
    impl From<S> for i32 {
        fn from(s: S) -> Self {
            s.i
        }
    }
    let expected = vec![11, 32, 34, 75, 21];
    let vector_1 = vec![S::new(1), S::new(3), S::new(3), S::new(7), S::new(2)];
    let vector_2 = vec![S::new(1), S::new(2), S::new(4), S::new(5), S::new(1)];
    assert_eq!(expected, function_2_vec(vector_1, vector_2));
}
