// Insired by
// https://exercism.org/tracks/rust/exercises/raindrops/solutions/jfmartin

fn is_factor(n: u32, factor: u32, word: &str) -> &str {
    if n % factor == 0 {
        word
    } else {
        ""
    }
}

pub fn raindrops(n: u32) -> String {
    let mut ans = String::new();
    ans.push_str(is_factor(n, 3, "Pling"));
    ans.push_str(is_factor(n, 5, "Plang"));
    ans.push_str(is_factor(n, 7, "Plong"));

    if ans.is_empty() {
        ans = format!("{}", n);
    }
    ans
}
