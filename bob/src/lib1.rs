fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
pub fn reply(message: &str) -> &str {
    let message = remove_whitespace(message);
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let has_letters = message.chars().any(|s| char::is_alphabetic(s));
    if has_letters {
        if message.clone().to_uppercase() == message.to_string() {
            if message.ends_with("?") {
                return "Calm down, I know what I'm doing!";
            }
            return "Whoa, chill out!";
        }
    }
    if message.ends_with("?") {
        return "Sure.";
    }
    "Whatever."
}
