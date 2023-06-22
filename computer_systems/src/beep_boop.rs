use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::thread::sleep;

pub fn beep_boop() {
    // read input from stdin with a message "Please type a number"
    let mut input = String::new();

    println!("Please type a number");

    stdin().read_line(&mut input).expect("Failed to read line");

    let input: u8 = input.trim().parse().expect("Please type a number");

    if input > 1 && input < 11 {
        for _ in 0..input {
            stdout().write_all(b"\x07").expect("beep failed");
            stdout().flush().expect("Failed to flush stdout");

            sleep(std::time::Duration::from_secs(2));
        }
    } else {
        println!("Please type a number between 1 and 10");
    }
}
