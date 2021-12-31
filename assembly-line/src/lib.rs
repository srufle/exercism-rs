// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_f64 = speed as f64;
    let rate = (221.0 * speed_f64);
    let rate = match speed {
        0 => 0.0,
        1..=4 => rate * 1.0,
        5..=8 => rate * 0.90,
        9..=u8::MAX => rate * 0.77,
    };
    rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    let items_per_minute = rate_per_hour / 60.0;
    items_per_minute as u32
}
