// Insired by
// https://exercism.org/tracks/rust/exercises/collatz-conjecture/solutions/damaxi

// Use of recursion, but fails for overflow tests
pub fn collatz_overflows(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
        n => collatz(3 * n + 1).map(|x| x + 1),
    }
}
pub fn collatz(n: u64) -> Option<u64> {
    let mut work: Option<u64> = Some(n);
    let mut steps: Option<u64> = Some(0);

    if work? == 0 {
        return None;
    } else if work? == 1 {
        return Some(0);
    } else {
        while let Some(x) = work {
            work = if x % 2 == 0 {
                let x = x / 2;
                Some(x)
            } else {
                let x: Option<u64> = 3u64.checked_mul(x).and_then(|i: u64| 1u64.checked_add(i));
                x
            };

            if let Some(s) = steps.as_mut() {
                *s += 1;
            }

            if work.is_none() {
                return None;
            } else if Some(1) == work {
                return steps;
            }
        }
    }

    steps
}
