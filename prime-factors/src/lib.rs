// Inspired by
// https://exercism.org/tracks/rust/exercises/prime-factors/solutions/SteelTermite
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;

    while number > 1 {
        for divisor in 2.. {
            while number % divisor == 0 {
                number /= divisor;
                factors.push(divisor);
            }
            if number <= 1 {
                break;
            }
        }
    }

    factors
}

pub fn factors_v1(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;
    let mut divisor = 2;

    while number > 1 {
        if number % divisor == 0 {
            factors.push(divisor);
            number = number / divisor;
        } else {
            divisor += 1
        }
    }

    factors
}
