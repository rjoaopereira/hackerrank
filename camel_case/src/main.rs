//https://www.hackerrank.com/challenges/camel_case
use std::io::{stdin};

fn main() {
    let mut string: String = String::new();
    stdin().read_line(&mut string).expect("invalid input");
    let count = string.trim().chars().fold(1, |acc, s:char| {
        if s.to_uppercase().collect::<Vec<char>>()[0] == s {
            return acc + 1;
        }
        return acc;
    });
    
    println!("{:?}", count);
}
