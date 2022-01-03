pub fn series_v0(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return Vec::new();
    } else if len >= digits.len() {
        return vec![(&digits[..]).to_string()];
    } else {
        let mut ans: Vec<String> = Vec::new();
        let mut start = 0;
        let mut end = start + len;
        while start < digits.len() + 1 {
            let s = &digits[start..end];

            ans.push(s.to_string());
            start = start + 1;
            end = start + len;
            if end > digits.len() {
                break;
            }
        }

        return ans;
    }
}

// https://exercism.org/tracks/rust/exercises/series/solutions/WeikangChen
pub fn series_WeikangChen(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}

// https://exercism.org/tracks/rust/exercises/series/solutions/de8ger
// from nederdirk
pub fn series_de8ger(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()
}
