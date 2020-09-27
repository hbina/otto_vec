#[test]
fn test_function_1() {
    let expected = vec![2, 5, 7, 12, 3];
    let vector_1 = vec![1, 3, 3, 7, 2];
    let vector_2 = vec![1, 2, 4, 5, 1];
    assert_eq!(
        expected,
        crate::functions::function_1_vec(vector_1, vector_2)
    );
}

#[test]
fn test_function_2() {
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
    assert_eq!(
        expected,
        crate::functions::function_2_vec(vector_1, vector_2)
    );
}

#[test]
fn test_function_3() {
    let expected = vec![Some(4), Some(4), None, None];
    let vector_1 = vec![Some(1), Some(2), Some(3), None];
    let vector_2 = vec![Some(3), Some(2), None, Some(0)];
    assert_eq!(
        expected,
        crate::functions::function_3_vec(vector_1, vector_2)
    );
}
