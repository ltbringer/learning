// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const BASE_PRODUCTION_RATE: f64 = 221.0;
    match speed {
        0 => 0.0,
        1..=4 => BASE_PRODUCTION_RATE * 1.0 * speed as f64,
        5..=8 => BASE_PRODUCTION_RATE * 0.9 * speed as f64,
        9..=10 => BASE_PRODUCTION_RATE * 0.77 * speed as f64,
        _ => panic!("Invalid speed: {}", speed),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / 60;
}
