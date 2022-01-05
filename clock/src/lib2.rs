use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

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
        println!("");

        let mut total_minutes = 0;
        let mut hours = dbg!(hours);
        let mut minutes = dbg!(minutes);

        // Calc hours from hours and what minutes contributes
        hours = dbg!(hours % HOURS_PER_DAY);

        if hours < 0 {
            hours = dbg!(hours + HOURS_PER_DAY);
        }
        // if minutes < 0 {
        //     minutes = dbg!(minutes % MINUTES_PER_HOUR);
        // }
        let hours_as_minutes = dbg!(hours * MINUTES_PER_HOUR);
        let hours_from_minutes = dbg!(minutes / MINUTES_PER_HOUR);
        let hours_from_minutes = if hours_from_minutes == 24 {
            0
        } else {
            hours_from_minutes
        };
        dbg!(hours_as_minutes);
        dbg!(hours_from_minutes);
        total_minutes = dbg!(hours_as_minutes + (hours_from_minutes * MINUTES_PER_HOUR));
        dbg!(total_minutes);

        minutes = dbg!(minutes % MINUTES_PER_HOUR);

        // if minutes < 0 {
        //     // minutes = dbg!(minutes + MINUTES_PER_HOUR);
        total_minutes = dbg!(total_minutes + minutes);
        // }
        // dbg!(hours_as_minutes + (hours_from_minutes * MINUTES_PER_HOUR) + minutes);
        dbg!(total_minutes);
        total_minutes
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        println!("new - hours: {},  minutes: {:?}", hours, minutes);
        let total_minutes = dbg!(Clock::normalize_time(hours, minutes));
        // let hour_minutes = Clock::hours_as_minutes(hours);
        dbg!(total_minutes);
        // let minutes = Clock::normalize_minutes(minutes);
        // println!("new - normalize_minutes: {}", minutes);
        // let total_minutes = hour_minutes + minutes;
        // println!("new - total_minutes: {}", total_minutes);
        // let (h, m) = Clock::minutes_to_time(total_minutes);
        // println!("new - minutes_to_time: h {} m {}", h, m);
        // let total_minutes = dbg!(h * MINUTES_PER_HOUR + m);
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
        let hours_total = minutes / MINUTES_PER_HOUR;
        let hours = hours_total % HOURS_PER_DAY;
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
            MINUTES_PER_DAY
        } else {
            hours = hours % HOURS_PER_DAY;
            if hours < 0 {
                hours = hours + HOURS_PER_DAY;
            }
            hours * MINUTES_PER_HOUR
        }
    }

    fn normalize_minutes(minutes: i32) -> i32 {
        let mut minutes = minutes;

        minutes = minutes % MINUTES_PER_HOUR;
        if minutes < 0 {
            minutes = minutes + MINUTES_PER_HOUR;
        }
        minutes
    }
}

fn normalize_time_orig(hours: i32, minutes: i32) -> i32 {
    let mut hours_as_minutes = 0;
    let mut hours = dbg!(hours);
    let mut minutes = dbg!(minutes);

    hours = hours % HOURS_PER_DAY;

    if hours < 0 {
        hours = hours + HOURS_PER_DAY;
    }

    minutes = minutes % MINUTES_PER_HOUR;
    if minutes < 0 {
        minutes = minutes + MINUTES_PER_HOUR;
    }

    // let overflow_hours = dbg!(minutes / MINUTES_PER_DAY);
    // hours = dbg!(hours + overflow_hours);
    // minutes = dbg!(minutes % MINUTES_PER_DAY);
    // let overflow_hours = dbg!(minutes / MINUTES_PER_HOUR);
    // hours = dbg!(hours + overflow_hours);
    // minutes = dbg!(minutes % MINUTES_PER_HOUR);

    // if hours < 0 && minutes < 0 {
    //     hours = dbg!(hours % HOURS_PER_DAY);
    //     hours_as_minutes = dbg!(hours * MINUTES_PER_HOUR);
    //     let addition_hours_from_minutes = dbg!(minutes / MINUTES_PER_HOUR);
    //     hours_as_minutes =
    //         dbg!(hours_as_minutes + addition_hours_from_minutes * MINUTES_PER_HOUR);
    //     hours_as_minutes = dbg!(hours_as_minutes + (minutes % MINUTES_PER_HOUR));
    //     hours_as_minutes = hours_as_minutes + MINUTES_PER_DAY;
    //     return hours_as_minutes;
    // }

    // if hours != 0 {
    //     hours = hours % HOURS_PER_DAY;
    //     if hours < 0 {
    //         hours = hours + HOURS_PER_DAY;
    //     }
    //     // hours * MINUTES_PER_HOUR
    // }
    // let hours_as_minutes = hours * MINUTES_PER_HOUR;

    // let mut minutes = minutes;

    // minutes = minutes % MINUTES_PER_HOUR;
    // if minutes < 0 {
    //     minutes = minutes + MINUTES_PER_HOUR;
    // }

    // let minutes = hours_as_minutes + minutes;
    // minutes
    return minutes;
}
