// https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust
pub fn high_and_low(numbers: &str) -> String {
    let (min, max) = numbers
        .split(' ')
        .map(|number| number.parse::<i32>().unwrap())
        .fold((i32::MIN, i32::MAX), |(max, min), number| {
            (max.max(number), min.min(number))
        });

    format!("{} {}", min, max)
}
