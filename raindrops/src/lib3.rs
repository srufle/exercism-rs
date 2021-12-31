// Insired by
// https://exercism.org/tracks/rust/exercises/raindrops/solutions/jfmartin
pub fn raindrops(n: u32) -> String {
    let is_factor = |factor, word| {
        if n % factor == 0 {
            word
        } else {
            ""
        }
    };
    let mut ans = String::new();
    ans.push_str(is_factor(3, "Pling"));
    ans.push_str(is_factor(5, "Plang"));
    ans.push_str(is_factor(7, "Plong"));

    if ans.is_empty() {
        ans = format!("{}", n);
    }
    ans
}

fn is_factor_v2(n: u32, factor: u32, word: &str) -> &str {
    if n % factor == 0 {
        word
    } else {
        ""
    }
}

pub fn raindrops_v2(n: u32) -> String {
    let mut ans = String::new();
    ans.push_str(is_factor_v2(n, 3, "Pling"));
    ans.push_str(is_factor_v2(n, 5, "Plang"));
    ans.push_str(is_factor_v2(n, 7, "Plong"));

    if ans.is_empty() {
        ans = format!("{}", n);
    }
    ans
}

pub fn raindrops_v1(n: u32) -> String {
    let pling = if n % 3 == 0 { "Pling" } else { "" };
    let plang = if n % 5 == 0 { "Plang" } else { "" };
    let plong = if n % 7 == 0 { "Plong" } else { "" };

    if pling.is_empty() && plang.is_empty() && plong.is_empty() {
        return format!("{}", n);
    } else {
        return format!("{}{}{}", pling, plang, plong);
    }
}
