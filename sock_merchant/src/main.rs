//https://www.hackerrank.com/challenges/sock_merchant
use std::io::{stdin};
use std::collections::HashMap;

fn main() {
    let mut items_input: String = String::new();
    stdin().read_line(&mut items_input).expect("invalid input");
    let items:i32 = items_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()[0];

    let mut colors_input: String = String::new();
    stdin().read_line(&mut colors_input).expect("invalid input");
    let colors:Vec<i32> = colors_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut pairs = HashMap::new();

    for color in &colors {
        let pair = pairs.entry(color).or_insert(0);
        *pair += 1;
    }

    println!("{:?}", pairs);

    let matches = pairs.iter().fold(0, |acc, (key, val)| {
        if val % 2 == 0 {
            return acc + val / 2;
        }
        return acc + (val - 1) / 2;
    });
    
    println!("{}", matches);
}
