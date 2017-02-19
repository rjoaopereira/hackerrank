//https://www.hackerrank.com/challenges/save-the-prisoner

use std::io::{stdin};

fn main() {
    let mut test_cases_number_input: String = String::new();
    stdin().read_line(&mut test_cases_number_input).expect("invalid input");
    let mut test_cases_number = test_cases_number_input.trim().parse::<u32>().unwrap_or(0);
    let mut results: Vec<u64> = Vec::new();
    while test_cases_number > 0 {
        let mut parameters_input: String = String::new();
        stdin().read_line(&mut parameters_input).expect("invalid input");
        let parameters: Vec<u64> = parameters_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let prisoners = parameters[0];
        let sweets = parameters[1];
        let i = parameters[2];
        test_cases_number -= 1;
        let mut rest;
        let mut pos;
        if sweets > prisoners {
            rest = (sweets % prisoners) - 1;
        } else {
            rest = sweets - 1;
        }
        pos = i + rest;
        if pos > prisoners {
            pos = pos % prisoners;
        }
        // println!("start: {}, pos: {}", i, pos); 
        results.push(pos);
    }
    for result in results {
        println!("{}", result);
    }
}

