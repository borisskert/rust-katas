/// https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust
///
/// # Arguments
///
/// * `numbers`: array of ten numbers to be formatted into a phone number
///
/// returns: String the formatted phone-number
///
pub fn create_phone_number(numbers: &[u8]) -> String {
    format!(
        "({}{}{}) {}{}{}-{}{}{}{}",
        numbers[0], numbers[1], numbers[2],
        numbers[3], numbers[4], numbers[5],
        numbers[6], numbers[7], numbers[8], numbers[9]
    )
}
