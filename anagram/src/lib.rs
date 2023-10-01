use std::collections::{HashMap, HashSet};

fn get_char_map_from_word(word: &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let char_map: HashMap<char, i32> = get_char_map_from_word(&word);
    possible_anagrams
        .iter()
        .filter(|ca| {
            let ca = ca.to_lowercase();
            ca != word && get_char_map_from_word(&ca) == char_map
        })
        .cloned()
        .collect::<HashSet<&'a str>>()
}
