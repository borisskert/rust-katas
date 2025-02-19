#[cfg(test)]
mod tests {
    use rust_katas::multiply::multiply;

    #[test]
    fn returns_expected_value() {
        test(3, 5, 15);
        test(-2, 1, -2);
        test(5, 0, 0);
    }

    fn test(a: i32, b: i32, expected: i32) {
        let actual = multiply(a, b);
        assert_eq!(
            actual, expected,
            "multiply({a}, {b}) should return {expected}, instead you returned {actual}\n"
        );
    }
}
