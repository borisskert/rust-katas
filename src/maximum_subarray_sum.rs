use std::cmp;

/// https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust
///
/// # Arguments
///
/// * `seq`: array if integers
///
/// returns: i32 the maximum sum of a contiguous subsequence in the specified array
///
pub fn max_sequence(seq: &[i32]) -> i32 {
    let (max, _) = seq
        .iter()
        .fold((0, 0), |(max, current), i| {
            let next_current = cmp::max(0, current + i);
            let next_max = cmp::max(next_current, max);

            (next_max, next_current)
        });

    max
}
