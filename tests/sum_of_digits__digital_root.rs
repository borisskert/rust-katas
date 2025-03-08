#[cfg(test)]
mod tests {
    use rust_katas::sum_of_digits_digital_root::digital_root;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(923403), 3);
        assert_eq!(digital_root(341345), 2);
        assert_eq!(digital_root(195), 6);
        assert_eq!(digital_root(992), 2);
        assert_eq!(digital_root(848939), 5);
        assert_eq!(digital_root(999999999999), 9);
        assert_eq!(digital_root(167346), 9);
        assert_eq!(digital_root(0), 0);
        assert_eq!(digital_root(650113), 7);
    }
}
