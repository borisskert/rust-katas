// https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust
pub fn get_middle(s:&str) -> &str {
    let len = s.len();
    let mid = len / 2;
    
    if len % 2 == 0 {
        &s[mid - 1..mid + 1]
    } else {
        &s[mid..mid + 1]
    }
}
