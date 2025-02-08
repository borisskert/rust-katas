#[cfg(test)]
mod tests {
    use rust_katas::square_every_digit::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
