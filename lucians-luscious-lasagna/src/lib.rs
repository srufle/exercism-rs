// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    println!("return expected minutes in the oven");
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected_minutes = expected_minutes_in_oven();
    println!(
        "calculate remaining minutes in oven given actual minutes in oven: {}",
        actual_minutes_in_oven
    );
    expected_minutes - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    println!(
        "calculate preparation time in minutes for number of layers: {}",
        number_of_layers
    );
    2 * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    println!(
        "calculate elapsed time in minutes for number of layers {} and actual minutes in oven {}",
        number_of_layers, actual_minutes_in_oven
    );
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
