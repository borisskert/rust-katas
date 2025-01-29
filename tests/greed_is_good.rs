#[cfg(test)]
mod tests {
    use rust_katas::greed_is_good::score;

    fn dotest(dice: [u8; 5], expected: u32) {
        let actual = score(dice);
        assert_eq!(
            actual, expected,
            "Expected score with dice {dice:?} to be {expected}, but was {actual}\n"
        );
    }

    #[test]
    fn sample_tests() {
        dotest([2, 3, 4, 6, 2], 0);
        dotest([4, 4, 4, 3, 3], 400);
        dotest([2, 4, 4, 5, 4], 450);
    }
}
