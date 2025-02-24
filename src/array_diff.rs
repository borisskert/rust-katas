// https://www.codewars.com/kata/523f5d21c841566fde000009/train/rust
pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for i in a {
        if !b.contains(&i) {
            result.push(i);
        }
    }

    result
}
