use std::{cmp::Ordering, fmt::Debug};

pub fn find<A: AsRef<[T]> + Debug, T: Ord + Debug>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    let array_len = array.len();
    if array_len == 0 {
        return None;
    }

    let mut min = 0;
    let mut max = array_len;
    let mut guess = array_len / 2;

    while guess < array_len {
        let val = &array[guess];
        if val.cmp(&key) == Ordering::Equal {
            return Some(guess);
        }

        if val.cmp(&key) == Ordering::Less {
            min = guess + 1;
        } else if val.cmp(&key) == Ordering::Greater {
            // deals with when no value exists
            // and you are already at element 0
            max = guess.checked_sub(1)?;
        }
        if max < min {
            return None;
        }
        guess = (min + max) / 2;
    }

    None
}
