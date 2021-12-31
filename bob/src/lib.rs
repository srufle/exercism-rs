// Inspired by
// https://exercism.org/tracks/rust/exercises/bob/solutions/mikechris
fn is_yelling(message: &str) -> bool {
    let has_letters = message.chars().any(|s| char::is_alphabetic(s));
    message.to_uppercase() == message && has_letters
}
pub fn reply(message: &str) -> &str {
    // match arm order matters, most specific to general
    match message.trim() {
        message if message.is_empty() => "Fine. Be that way!",
        message if message.ends_with("?") && is_yelling(message) => {
            "Calm down, I know what I'm doing!"
        }
        message if message.ends_with("?") => "Sure.",
        message if is_yelling(message) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
