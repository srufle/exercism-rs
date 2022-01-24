pub fn abbreviate(phrase: &str) -> String {
    dbg!(phrase);
    // unimplemented!("Given the phrase '{}', return its acronym", phrase);
    let split = phrase
        // .split(|c| (char::is_whitespace(c) || char::is_ascii_punctuation(&c)));
        .split(&[' ', '-', '_']);
    let ans = split
        .into_iter()
        .map(|word| word.chars().nth(0).unwrap_or_default())
        // .map(|word| word.chars().filter(|c| c.is_uppercase()))
        .map(|c| {
            if c.is_alphabetic() {
                c.to_uppercase().to_string()
            } else {
                String::new()
            }
        })
        .collect::<String>();

    println!("ans: {:?}", ans);

    ans
}

pub fn abbreviate_v1(phrase: &str) -> String {
    // unimplemented!("Given the phrase '{}', return its acronym", phrase);
    let ans = phrase
        .chars()
        .into_iter()
        .filter(|c| c.is_uppercase())
        .collect::<String>();
    ans
}
