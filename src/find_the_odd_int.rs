// https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust
pub fn find_odd(arr: &[i32]) -> i32 {
    let mut result = 0;
    
    for i in arr {
        result ^= i; // XOR
    }
    
    result
}
