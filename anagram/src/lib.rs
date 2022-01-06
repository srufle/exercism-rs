use std::collections::HashSet;

// After mentor bobahop reply
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Only need to do this once since word is used in all other comparisons
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);

    // for each possible word check its lower case version against
    // the "word" passed in, if the length and sorted value is the same,
    // BUT is NOT the "word" it IS an anagram.
    possible_anagrams
        .iter()
        // .cloned()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && get_sorted(&candidate_lower) == word_sorted
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
