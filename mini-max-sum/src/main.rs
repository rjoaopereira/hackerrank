//https://www.hackerrank.com/challenges/mini-max-sum
use std::io::{stdin};

fn main() {
    let mut numbers_input: String = String::new();
    stdin().read_line(&mut numbers_input).expect("Did not enter a correct string");
    let mut numbers: Vec<i64> = numbers_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    numbers.sort();
    let max: i64 = numbers[..4].iter().fold(0, |acc, curr| acc + curr);
    let min: i64 = numbers[1..].iter().fold(0, |acc, curr| acc + curr);
    println!("{} {}", max, min);
}
