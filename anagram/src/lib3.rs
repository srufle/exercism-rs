use std::collections::HashSet;
// Inspired by
// https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen
pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut ans = HashSet::new();
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    for possible in possible_anagrams.iter().cloned() {
        // dbg!(&word, possible);
        if word.len() == possible.len() {
            let possible = possible.to_lowercase();
            let possible_sorted = sort(&possible);

            if possible_sorted == sorted_word {
                dbg!(word, possible);
                if word != possible {
                    // let p = possible.to_owned();
                    // let p = &p[..];
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
pub fn anagrams_for_good<'a>(word: &str, candidates: &'a [&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort(&word);
    candidates
        .iter()
        .cloned()
        .filter(|&candidate| {
            let can = candidate.to_lowercase();
            sort(&can) == sorted_word && can != word
        })
        .collect::<HashSet<&'a str>>()
}

// Inspired by
// https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen
pub fn anagrams_for_rufle<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
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
