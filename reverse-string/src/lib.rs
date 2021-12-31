use unicode_segmentation::UnicodeSegmentation;

// Used https://crates.io/crates/unicode-segmentation
pub fn reverse(input: &str) -> String {
    // looked at
    // https://exercism.org/tracks/rust/exercises/reverse-string/solutions/kmorrissey-mersive
    input
        .graphemes(true)
        .rev()
        .flat_map(|c| c.chars())
        .collect()
}
pub fn reverse_v1(input: &str) -> String {
    let mut g: Vec<&str> = UnicodeSegmentation::graphemes(input, true).collect();
    g.reverse();
    let ans: String = g.into_iter().collect();
    ans
}
