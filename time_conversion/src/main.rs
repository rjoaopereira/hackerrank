//https://www.hackerrank.com/challenges/time-conversion
use std::io::{stdin};
fn main() {

    let mut time=String::new();
    let format;

    stdin().read_line(&mut time).expect("Did not enter a correct string");
    format = &time[8..10];
    let mut hours = time[0..2].parse::<i32>().unwrap();
    let mut hours_str = String::new();

    if format == "AM" {
        if hours == 12 {
            hours_str.push_str("00")
        } else {
            if hours < 10 {
                hours_str.push_str("0");
                hours_str.push_str(&hours.to_string())
            } else {
                hours_str.push_str(&hours.to_string())
            }
        }
    } else {
        if hours < 12 {
            hours += 12
        }
        hours_str.push_str(&hours.to_string())
    }


    println!("{}{}",hours_str, &time[2..8])
}