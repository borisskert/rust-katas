/// https://www.codewars.com/kata/555624b601231dc7a400017a/train/rust
pub fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut x = 2;
    let mut i = 0;

    while i < n {
        i += 1;

        x = x + k - 1;
        x = x % i + 1;
    }

    x
}
