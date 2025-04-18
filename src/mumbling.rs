// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust
pub fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            let word = c.to_string().repeat(i + 1);
            capitalize(word)
        })
        .collect::<Vec<String>>()
        .join("-")
}

fn capitalize(s: String) -> String {
    let first = s.chars().next().unwrap();
    let rest = s.chars().skip(1).collect::<String>();

    format!("{}{}", first.to_uppercase(), rest.to_lowercase())
}
