use std::ops::Neg;

// https://www.codewars.com/kata/55685cd7ad70877c23000102/train/rust
pub fn make_negative(n: i32) -> i32 {
    n.abs().neg()
}
