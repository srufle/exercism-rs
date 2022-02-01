// Inspired By
// https://exercism.org/tracks/rust/exercises/binary-search/solutions/SpyR1014
pub fn find<A, T>(array: A, key: T) -> Option<usize>
where
    A: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    let mut min = 0;
    let mut max = array.len();

    while min < max {
        let guess = (min + max) / 2;
        let val = &array[guess];
        if val > &key {
            max = guess
        } else if val < &key {
            min = guess + 1;
        } else {
            return Some(guess);
        }
    }

    None
}
