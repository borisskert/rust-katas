// https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust
pub fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&&x| x > 0).sum()
}
