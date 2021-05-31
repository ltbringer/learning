use std::ops::{Add};
use std::time::{Duration};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(3.4, 4.5);
    let ints = add(1, 1);
    let durations = add(Duration::new(50, 7000), Duration::new(10, 550));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}
