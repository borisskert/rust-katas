use itertools::Itertools;

///
/// https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust
/// # Arguments
///
/// * `words`: String containing the words to be spun
///
/// returns: String containing the spun words
///
pub fn spin_words(words: &str) -> String {
    words.split(' ')
        .map(spin_word)
        .join(" ")
}

fn spin_word(word: &str) -> String {
    if word.len() >= 5 {
        return word.chars()
            .rev()
            .collect();
    }

    String::from(word)
}
