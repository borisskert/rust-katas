// https://www.codewars.com/kata/546e2562b03326a88e000020/train/rust
pub fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d * d)
        .map(|d| d.to_string())
        .fold("".to_string(), |acc, d| acc + &d)
        .parse::<u64>()
        .unwrap()
}
