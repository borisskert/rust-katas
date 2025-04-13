#[cfg(test)]
mod tests {
    use rust_katas::convert_a_number_to_a_string::number_to_string;

    fn dotest(n: i32, expected: &str) {
        let actual = number_to_string(n);
        assert_eq!(
            actual, expected,
            "With n = {n}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(67, "67");
        dotest(79585, "79585");
        dotest(1 + 2, "3");
        dotest(1 - 2, "-1");
        dotest(0, "0");
    }
}
