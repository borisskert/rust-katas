/// https://www.codewars.com/kata/55aa075506463dac6600010d/train/rust
pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n)
        .map(|i| (i, divisors(i)))
        .map(|(i, divisors)| (i, square_sum(divisors)))
        .filter(|(_, sum)| is_square(*sum))
        .collect()
}

fn divisors(n: u64) -> Vec<u64> {
    (1..=n)
        .take_while(|i| i * i <= n)
        .filter(|i| n % i == 0)
        .flat_map(|i| if i * i == n { vec![i] } else { vec![i, n / i] })
        .collect()
}

fn square_sum(nums: Vec<u64>) -> u64 {
    nums.into_iter().map(|n| n * n).sum()
}

fn is_square(n: u64) -> bool {
    (n as f64).sqrt().fract() == 0.0
}
