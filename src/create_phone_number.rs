use std::fmt::{format};

/// https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust
///
/// # Arguments
///
/// * `numbers`: array of ten numbers to be formatted into a phone number
///
/// returns: String the formatted phone-number
///
/// # Examples
///
/// ```rust,ignore
/// create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
/// // -> "(123) 456-7890"
/// ```
fn create_phone_number(numbers: &[u8]) -> String {
    return format!(
        "({}{}{}) {}{}{}-{}{}{}{}",
        numbers[0], numbers[1], numbers[2],
        numbers[3], numbers[4], numbers[5],
        numbers[6], numbers[7], numbers[8], numbers[9]
    );
}

#[test]
fn returns_expected() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}
