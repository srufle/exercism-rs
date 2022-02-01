pub fn find(array: &[i32], key: i32) -> Option<usize> {
    dbg!(array, key);
    let array_len = array.len();
    if array_len == 0 {
        return None;
    }

    let mut min = 0;
    let mut max = array_len;
    let mut guess = array_len / 2;

    while guess < array_len {
        let val = array[guess];
        dbg!(guess, val, key);
        if val == key {
            return Some(guess);
        }

        if val < key {
            min = guess + 1;
        } else if val > key {
            // deals with when no value exists
            // and you are already at element 0
            max = guess.checked_sub(1)?;
        }
        dbg!(min, max, guess);
        if max < min {
            return None;
        }
        guess = (min + max) / 2;
    }

    None
}
