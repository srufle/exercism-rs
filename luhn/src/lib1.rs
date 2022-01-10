/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    dbg!(code);
    let code = code.replace(" ", "");
    if !code
        .chars()
        .all(|c| c.is_numeric() || c.is_ascii_whitespace())
        || code.len() < 2
    {
        return false;
    }
    let all_digits = code
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10))
        .flat_map(|n| n)
        .collect::<Vec<_>>();
    // dbg!(all_digits);

    let all_digits = all_digits.as_slice();
    let mut new_digits: Vec<u32> = Vec::new();
    let mut index = all_digits.len() - 1;
    let mut position = 1;
    while index >= 0 {
        let d = all_digits[index];
        dbg!(d);
        let n = if position % 2 == 0 {
            let mut n = d * 2;
            let n = if n > 9 {
                n = n - 9;
                n
            } else {
                n
            };
            n
        } else {
            d
        };
        dbg!(n);
        new_digits.push(n);
        if index == 0 {
            break;
        }
        index -= 1;
        position += 1;
    }

    dbg!(&new_digits);

    let total = dbg!(new_digits.iter().sum::<u32>());
    total % 10 == 0
}
