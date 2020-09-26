# otto_vec

> Automatically vectorize any pure functions!

If a function is pure, this procedural macro should be able to vectorize it for free.

## Example

```rust
#[otto_vec]
pub fn function_2<X: Into<i32>, Y: Into<i32>>(a: X, b: Y) -> i32 {
    let result = 10;
    result * a.into() + b.into()
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
```

## Todo

1.  Make retaining the original function optional --- In general, we want more configurations available.