use std::fmt;
#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

// Insired by
// https://exercism.org/tracks/rust/exercises/clock/solutions/ukolka
// Really struggled with the wrapping of time and negative time
const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m) = Clock::minutes_to_time(self.minutes);
        write!(f, "{:02}:{:02}", h, m)
    }
}
impl Clock {
    fn normalize_time(hours: i32, minutes: i32) -> i32 {
        let total_minutes = hours * MINUTES_PER_HOUR + minutes;
        let total_minutes = total_minutes.rem_euclid(MINUTES_PER_DAY);
        let total_minutes = total_minutes + MINUTES_PER_DAY;
        let total_minutes = total_minutes.rem_euclid(MINUTES_PER_DAY);
        total_minutes
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = Clock::normalize_time(hours, minutes);

        Clock {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        // Had the self.minutes + minutes.
        // Key insight is that when you can create
        // a new clock correctly using just minutes
        // This part is solved
        Clock::new(0, minutes)
    }

    fn minutes_to_time(minutes: i32) -> (i32, i32) {
        let hours = minutes.div_euclid(MINUTES_PER_HOUR);
        let minutes = minutes.rem_euclid(MINUTES_PER_HOUR);
        (hours, minutes)
    }
}
