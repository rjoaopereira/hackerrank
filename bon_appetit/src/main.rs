//https://www.hackerrank.com/challenges/bon-appetit
use std::io::{stdin};

fn main() {
    let mut items_input: String = String::new();
    stdin().read_line(&mut items_input).expect("invalid input");
    let items:Vec<u64> = items_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let n = items[0];
    let k = items[1];

    let mut cost_input: String = String::new();
    stdin().read_line(&mut cost_input).expect("invalid input");
    let cost:Vec<f64> = cost_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();

    let mut charged_input: String = String::new();
    stdin().read_line(&mut charged_input).expect("invalid input");
    let charged:f64 = charged_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>()[0];

    let bill = cost.iter().enumerate().fold(0.0, |acc, (i, price)| {
        if i != k as usize{
            return acc + price / 2.0;
        }
        return acc;
    });

    if bill == charged {
        println!("Bon Appetit");
    } else {
        println!("{}", charged - bill);
    }
}
