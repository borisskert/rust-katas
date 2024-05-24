use itertools::Itertools;

///
/// https://www.codewars.com/kata/523a86aa4230ebb5420001e1/train/rust
/// # Arguments
///
/// * `word`: the specified word as &str
/// * `words`: the list of words searching for anagrams
///
/// returns: Vec<String, Global>
///
/// # Changelog
/// * Improve performance by checking string-length before sorting
pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let sorted = sort_letters(&String::from(word));

    return words.iter()
        .filter(|x| sort_letters(x) == sorted)
        .map(String::from)
        .collect();
}

fn sort_letters(word: &str) -> String {
    return word.chars()
        .sorted()
        .collect();
}
