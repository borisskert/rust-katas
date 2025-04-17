// https://www.codewars.com/kata/54c27a33fb7da0db0100040e/train/rust
pub fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }

    let sqrt = (n as f64).sqrt().floor() as i64;

    sqrt * sqrt == n
}
