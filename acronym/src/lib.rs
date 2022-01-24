// Inspired by
// https://exercism.org/tracks/rust/exercises/acronym/solutions/menski
pub fn abbreviate(phrase: &str) -> String {
    // Split on separators
    // flat_map over words
    // for each word, take the first letter,
    // reducing the word by one letter.
    // then skipping any CAPITAL letters in the same word (GNU)
    // then filtering only CAPITAL letters (CamelCase)
    // Collect into a string and uppercase the result
    let ans = phrase
        .split(&[' ', '-', '_'])
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase();

    ans
}
