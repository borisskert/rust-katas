// https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust
pub fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect()
}
