// https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust
pub fn digital_root(n: i64) -> i64 {
    if n < 10 {
        return n;
    }

    let sum = digits_of(n).iter().sum();

    digital_root(sum)
}

fn digits_of(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut digits = Vec::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();

    digits
}
