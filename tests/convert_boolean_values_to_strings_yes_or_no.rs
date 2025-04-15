#[cfg(test)]
mod tests {
    use rust_katas::convert_boolean_values_to_strings_yes_or_no::bool_to_word;

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
}
