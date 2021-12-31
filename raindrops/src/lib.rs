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
