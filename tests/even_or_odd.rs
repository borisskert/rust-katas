#[cfg(test)]
mod sample_tests {
    use rust_katas::even_or_odd::even_or_odd;

    fn do_test(number: i32, expected: &str) {
        let actual = even_or_odd(number);
        assert_eq!(actual, expected, "\nYour result (left) does not match the expected output (right) for the input {number:?}");
    }

    #[test]
    fn test_zero() {
        do_test(0, "Even");
    }

    #[test]
    fn test_positive_even() {
        do_test(2, "Even");
    }

    #[test]
    fn test_positive_odd() {
        do_test(1, "Odd");
    }

    #[test]
    fn test_negative_even() {
        do_test(-2, "Even");
    }

    #[test]
    fn test_negative_odd() {
        do_test(-1, "Odd");
    }
}
