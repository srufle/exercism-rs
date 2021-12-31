// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();
    // Redoing according to https://exercism.org/tracks/rust/exercises/magazine-cutout/solutions/Marval13
    for word in magazine {
        // check map and insert 0 if it does not exist, returns mutable reference
        let entry = magazine_words.entry(word).or_insert(0);
        // increment count
        *entry += 1;
    }

    for word in note {
        // Now look in magazine words for note word and see if the count is great
        // enough to fulfill requirement
        let entry = magazine_words.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        // deduct from count
        *entry -= 1;
    }

    true
}

fn create_counts_v1(input: &[&str]) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    for key in input {
        match counts.get(&key.to_string()) {
            Some(word_count) => {
                let word_count = word_count + 1;
                counts.insert(key.to_string(), word_count);
            }
            None => {
                let _ = counts.insert(key.to_string(), 1);
            }
        };
    }
    counts
}
pub fn can_construct_note_v1(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_counts = create_counts_v1(magazine);
    let mut note_counts = create_counts_v1(note);

    for note_key in note_counts.keys() {
        let magazine_count = magazine_counts.get(note_key).unwrap_or(&0);
        let note_count = note_counts.get(note_key).unwrap();

        if note_count > magazine_count {
            return false;
        }
    }
    true
}
