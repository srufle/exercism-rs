//  Insired By
// https://exercism.org/tracks/rust/exercises/series/solutions/de8ger
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    // chars returns an iterator,
    // collect will turn an iterator
    // into a collection -> Vec of chars
    // windows only works on slices, create a window of len
    // collect the window chars and turn into a String using map
    // then collect the strings into a Vec of strings
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|s| s.into_iter().collect::<String>())
        .collect()
}
