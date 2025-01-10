/// https://www.codewars.com/kata/5917fbed9f4056205a00001e/train/rust
pub fn bananas(s: &str) -> Vec<String> {
    search(s, "banana")
}

fn search(text: &str, banana: &str) -> Vec<String> {
    if banana.is_empty() {
        return vec![text.chars().map(|_| '-').collect()];
    }

    if text.is_empty() {
        return Vec::new();
    }

    if head_of(banana) == head_of(text) {
        return take_and_omit(text, banana);
    }

    omit_one(text, banana)
}

fn take_and_omit(text: &str, banana: &str) -> Vec<String> {
    let omitted: Vec<String> = omit_one(text, banana);
    let taken: Vec<String> = take_one(text, banana);

    omitted.iter()
        .chain(taken.iter())
        .cloned()
        .collect()
}

fn take_one(text: &str, banana: &str) -> Vec<String> {
    let char = banana.chars().next().unwrap();

    search(&text[1..], &banana[1..])
        .iter()
        .map(|x| format!("{}{}", char, x))
        .collect()
}

fn omit_one(text: &str, banana: &str) -> Vec<String> {
    search(&text[1..], banana)
        .iter()
        .map(|x| format!("-{}", x))
        .collect()
}

fn head_of(s: &str) -> char {
    s.chars().next().unwrap()
}
