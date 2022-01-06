use std::collections::HashSet;
// Inspired by
// https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ans = HashSet::new();
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    for possible in possible_anagrams.iter().cloned() {
        let poss = possible.to_lowercase();
        let poss_sorted = sort(&poss);

        if poss_sorted == sorted_word {
            if poss != word {
                ans.insert(possible);
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
