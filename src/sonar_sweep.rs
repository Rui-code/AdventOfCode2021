use std::io::{stdin, prelude::*};

pub fn sonar_sweep() {
    let mut current: i32;
    let mut increases = 0;
    let mut previous = 0;

    for line in stdin().lock().lines() {
        current = line
            .unwrap()
            .trim()
            .parse()
            .expect("This is not a number.");

        if current > previous && previous != 0 {
            increases += 1;
        }
        previous = current;

    }

    println!("\n{}", increases);
}