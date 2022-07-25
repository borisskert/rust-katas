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
fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let sorted = sort_letters(&String::from(word));

    return words.iter()
        .filter(|x| sort_letters(x) == sorted)
        .map(|x| String::from(x))
        .collect();
}

fn sort_letters(word: &String) -> String {
    return word.chars()
        .sorted()
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);

        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
