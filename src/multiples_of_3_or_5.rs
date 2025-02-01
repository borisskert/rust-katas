// https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust
pub fn solution(num: i32) -> i32 {
    (0..num)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}
