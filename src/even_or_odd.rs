// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust
pub fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        _ => "Odd",
    }
}
