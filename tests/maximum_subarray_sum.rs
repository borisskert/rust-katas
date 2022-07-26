#[cfg(test)]
mod tests {
    use rust_katas::maximum_subarray_sum::max_sequence;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}
