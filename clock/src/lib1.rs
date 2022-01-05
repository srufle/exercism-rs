use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const DAY_AS_MINUTES: i32 = HOURS_IN_DAY * MINUTES_IN_HOUR;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m) = Clock::minutes_to_time(self.minutes);
        write!(f, "{:02}:{:02}", h, m)
    }
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        println!("new - hours: {},  minutes: {:?}", hours, minutes);
        let hour_minutes = Clock::hours_as_minutes(hours);
        println!("new - hour_minutes: {}", hour_minutes);
        let minutes = Clock::normalize_minutes(minutes);
        println!("new - normalize_minutes: {}", minutes);
        let total_minutes = hour_minutes + minutes;
        println!("new - total_minutes: {}", total_minutes);
        let (h, m) = Clock::minutes_to_time(total_minutes);
        println!("new - minutes_to_time: h {} m {}", h, m);
        let total_minutes = h * MINUTES_IN_HOUR + m;
        Clock {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!(
            "add_minutes - self minutes: {}, plus minutes: {:?}",
            self.minutes, minutes
        );
        let minutes = self.minutes + minutes;

        println!("add_minutes minutes: {}", minutes);
        Clock { minutes }
    }

    fn minutes_to_time(minutes: i32) -> (i32, i32) {
        println!("minutes_to_time - minutes in: {}", minutes);
        let hours_total = minutes / MINUTES_IN_HOUR;
        let hours = hours_total % HOURS_IN_DAY;
        let minutes = minutes - (hours_total * 60);
        println!(
            "minutes_to_time - hours out: {}, minutes out:{}",
            hours, minutes
        );

        (hours, minutes)
    }
    fn hours_as_minutes(hours: i32) -> i32 {
        let mut hours = hours;
        if hours == 0 {
            DAY_AS_MINUTES
        } else {
            hours = hours % HOURS_IN_DAY;
            if hours < 0 {
                hours = hours + HOURS_IN_DAY;
            }
            hours * MINUTES_IN_HOUR
        }
    }

    fn normalize_minutes(minutes: i32) -> i32 {
        let mut minutes = minutes;

        minutes = minutes % MINUTES_IN_HOUR;
        if minutes < 0 {
            minutes = minutes + MINUTES_IN_HOUR;
        }
        minutes
    }

    fn hours_after(hours: i32) -> i32 {
        // doing twice deals with negitive time
        (hours % 24) + 24 % 24
    }
}
