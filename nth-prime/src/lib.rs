// Using trial division method to test primality
// https://primes.utm.edu/glossary/xpage/TrialDivision.html
// https://inventwithpython.com/cracking/chapter22.html
// https://en.wikipedia.org/wiki/Primality_test

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let nf: f64 = n as f64;
    for i in 2..(f64::sqrt(nf) as u32) + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Insired by
// https://exercism.org/tracks/rust/exercises/nth-prime/solutions/plippe
pub fn nth(n: u32) -> u32 {
    (1..).filter(|p| is_prime(*p)).nth(n as usize).unwrap()
}

pub fn nth_v1(n: u32) -> u32 {
    let mut current_number: u32 = 0;
    let mut current_prime: u32 = 0;
    while current_prime <= n {
        current_number += 1;
        if is_prime(current_number) {
            current_prime += 1;
        }
    }
    current_number
}
