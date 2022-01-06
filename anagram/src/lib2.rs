use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
// Inspired by
// https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen
pub fn anagrams_for_v1<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ans = HashSet::new();
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    // for p as &&str in possible_anagrams.iter() {
    // for p as &str in possible_anagrams.iter().cloned() {
    for possible in possible_anagrams.iter().cloned() {
        dbg!(&word, possible);
        if word.len() == possible.len() {
            // let w1 = word.to_lowercase();
            // let w1 = sort(&w1);
            let possible = possible.to_lowercase();
            let possible_sorted = sort(&possible);
            // let mut w1: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
            // w1.sort_unstable();
            // let mut w2: Vec<&str> = UnicodeSegmentation::graphemes(*p, true).collect();
            // w2.sort_unstable();
            if possible_sorted == sorted_word {
                dbg!(word, possible);
                if word != possible {
                    ans.insert(possible);
                }
            }
        }
    }
    ans
}

fn sort(word: &str) -> Vec<char> {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted
}

#[allow(dead_code)]
pub fn anagrams_for_v2<'a>(word: &str, candidates: &'a [&'a str]) -> Vec<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    candidates
        .iter()
        .cloned()
        .filter(|&candidate| {
            let can = candidate.to_lowercase();
            sort(&can) == sorted_word && can != word
        })
        .collect::<Vec<&'a str>>()
}
