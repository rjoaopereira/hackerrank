//https://www.hackerrank.com/challenges/divisible_sum_pairs
use std::io::{stdin};

fn main() {
    let mut rules_input: String = String::new();
    stdin().read_line(&mut rules_input).expect("invalid input");
    let rules:Vec<u32> = rules_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let k = rules[1];

    let mut int_input: String = String::new();
    stdin().read_line(&mut int_input).expect("invalid input");
    let ints:Vec<u32> = int_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut pos = 0;
    let mut temp = pos;
    let count = ints.iter().fold(0 , |acc, &x| {
        temp = pos;
        pos += 1;
        if pos < ints.len() {
            return ints[(temp + 1)..ints.len()].iter().fold(acc, |acc2, &y| {
                if (x + y) % k == 0 {
                    return acc2 + 1
                }
                return acc2
            });
        }
        return acc
    });
    
    println!("{}", count);
}
