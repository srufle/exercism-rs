// https://nicholassterling.wordpress.com/2018/08/31/o1-sum_of_multiples-in-rust/
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // n * each factor, only keeping items that divide evenly
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
    // Inspired By:
    // https://exercism.org/tracks/rust/exercises/sum-of-multiples/solutions/Wow-BOB-Wow
    // https://exercism.org/tracks/rust/exercises/sum-of-multiples/solutions/794736b7feb94c8f8519214b8cfddcc8
    (1..limit)
        .filter(|n| {
            factors
                .iter()
                // remove the zero factor
                .filter(|fac| fac > &&0)
                .any(|fac| n % fac == 0)
        })
        .sum()
}
