use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
// https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ans = HashSet::new();
    // let word = word.to_lowercase();
    // let sorted_word = sort(&word);
    // let word = &word[..];
    for p in possible_anagrams.iter() {
        dbg!(word, p);
        if word.len() == p.len() {
            // let mut w1: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();
            // w1.sort_unstable();

            // let p = p.to_string().to_lowercase();
            // let p = &&p[..];
            // let mut w2: Vec<&str> = UnicodeSegmentation::graphemes(*p, true).collect();
            // w2.sort_unstable();
            let w2 = sort(p.clone());
            if sorted_word == w2 {
                dbg!(word, w2);
                if &word != *p {
                    ans.insert(*p);
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
