use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Anagram {
    word: String,
}

impl Anagram {
    pub fn new(input: &str) -> Self {
        let mut chars:Vec<char> = input.to_lowercase().chars().collect();
        chars.sort();
        Anagram { word: String::from_iter(chars) }
    }
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let origin = Anagram::new(word);
    possible_anagrams
        .iter()
        .filter(|w| {
            origin == Anagram::new(w)
                && w.to_lowercase() != word.to_lowercase()
        })
        .copied()
        .collect()
}
