/// Check a Luhn checksum.
// Inspired By
// https://exercism.org/tracks/rust/exercises/luhn/solutions/JaneL
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold
// https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (sum + num, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
// str to chars reverse let anything NOT whitespace through (digits and maybe symbols)
// try and process digits as you go
// parse/convert to a u32 digit
// a series of map calls take a value and process according to luhn rules
// or allows None to pass to the end to return false, otherwise
// it checks sum and count
