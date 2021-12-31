// Look at
// https://exercism.org/tracks/rust/exercises/leap/solutions/dastbe
pub fn is_leap_year(year: u64) -> bool {
    let every_4 = year % 4 == 0;
    let every_100 = year % 100 == 0;
    let every_400 = year % 400 == 0;

    if every_4 && every_100 && every_400 {
        return true;
    } else if every_4 && every_100 {
        return false;
    } else if every_4 {
        return true;
    }

    false
}
