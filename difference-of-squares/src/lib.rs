pub fn square_of_sum(n: u32) -> u32 {
    let ans: u32 = (1..=n).sum::<u32>().pow(2);
    ans
}

pub fn sum_of_squares(n: u32) -> u32 {
    let ans: u32 = (1..=n).map(|n| u32::pow(n, 2)).sum();
    ans
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
