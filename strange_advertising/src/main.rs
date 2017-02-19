//https://www.hackerrank.com/challenges/strange-advertising

use std::io::{stdin};

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("invalid input");
    let mut days = input.trim().parse::<u32>().unwrap();
    let mut people:f64 = 5.0;
    let mut liked:f64  = 0.0;
    while days > 0 {
        liked += (people/2.0).floor();
        people = (people/2.0).floor() * 3.0;
        days -= 1;
    }
    
    println!("{:?}", liked);
}
