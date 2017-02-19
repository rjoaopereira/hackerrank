//https://www.hackerrank.com/challenges/two_characters
//input
//10
//beabeefeab
//solution
//5
use std::io::{stdin};
use std::cmp;

fn verify_string(stripped_str: &str) -> u64 {
    if stripped_str.len() <= 2 {
        return if stripped_str.len() == 2 { 2 } else { 0 } ;
    }
    let mut string = stripped_str.split("").collect::<Vec<&str>>();
    string.pop();
    string.remove(0);
    let mut iter = string.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    if first == second {
        return 0;
    }
    let verified = string.iter().enumerate().all(|(i, s)| {
        if i % 2 == 0 {
            return s == first;
        }
        return s == second;
    });
    if verified {
        return stripped_str.len() as u64;
    }
    return 0;
}

fn main() {
    let mut length: String = String::new();
    stdin().read_line(&mut length).expect("invalid input");

    let mut string_input: String = String::new();
    stdin().read_line(&mut string_input).expect("invalid input");
    let string = string_input.trim();
    
    let mut chars = string.split("").collect::<Vec<&str>>();
    chars.pop();
    chars.remove(0);
    chars.sort();
    chars.dedup();
    let mut max = 0;
    
    for x in 0..chars.len() {
        for y in 0..chars.len() {
            if x == y {
                continue;
            }
            let temp_string = string.chars().fold(String::new(), |mut prev, chr| {
                if chars[x] == chr.to_string() || chars[y] == chr.to_string() {
                    prev.push(chr);
                }
                return prev;
            });
            max = cmp::max(max, verify_string(&temp_string));
        }   
    }

    println!("{:?}", max);
}
