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
