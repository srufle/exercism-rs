// Inspired by
// https://exercism.org/tracks/rust/exercises/proverb/solutions/cbzehner
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|sub_slice| {
            let want = sub_slice[0];
            let lost = sub_slice[1];
            format!("For want of a {0} the {1} was lost.", want, lost)
        })
        .chain(std::iter::once(format!(
            "And all for the want of a {0}.",
            list[0]
        )))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn build_proverb_v1(list: &[&str]) -> String {
    let mut proverb: Vec<String> = Vec::new();
    let original_want = if list.len() > 0 {
        list[0]
    } else {
        return String::from("");
    };
    for index in 0..list.len() - 1 {
        let want = list[index];
        let lost = list[index + 1];

        proverb.push(format!("For want of a {0} the {1} was lost.", want, lost));
    }

    proverb.push(format!("And all for the want of a {0}.", original_want));
    proverb.join("\n")
}
