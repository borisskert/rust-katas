// https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/rust
pub fn get_count(string: &str) -> usize {
    string.chars().filter(|c| "aeiou".contains(*c)).count()
}
