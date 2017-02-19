//https://www.hackerrank.com/challenges/designer-pdf-viewer?h_r=next-challenge&h_v=zen
use std::io::{stdin};
use std::cmp;

fn main() {
    let letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut numbers_input: String = String::new();
    stdin().read_line(&mut numbers_input).expect("Did not enter a correct string");
    let numbers: Vec<i32> = numbers_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut word_input: String = String::new();
    stdin().read_line(&mut word_input).expect("Did not enter a correct string");
    let word = word_input.trim().split("").filter(|&s| s != "").collect::<Vec<&str>>();
    let mut count: usize = 0;
    let mut height: i32 = 0;
    while count < word.len() {
        height = cmp::max(height, numbers[letters.iter().position(|&x| x == word_input.chars().nth(count).unwrap()).unwrap()]);
        count += 1;
    }
    let area = height * (word.len() as i32);
    println!("{}", area);
}
