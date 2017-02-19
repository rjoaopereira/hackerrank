//https://www.hackerrank.com/challenges/find_digits

use std::io::{stdin};

fn main() {
    let mut test_cases_number_input: String = String::new();
    stdin().read_line(&mut test_cases_number_input).expect("invalid input");
    let mut test_cases_number = test_cases_number_input.trim().parse::<u32>().unwrap_or(0);
    let mut results: Vec<u64> = Vec::new();
    while test_cases_number > 0 {
        let mut numbers_input: String = String::new();
        stdin().read_line(&mut numbers_input).expect("invalid input");
        let number: u64 = numbers_input.trim().parse::<u64>().unwrap();
        let digits: Vec<u64> = numbers_input.trim().chars().map(|x| x.to_string().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let zero:u64 = 0;
        let divisible = digits.iter().fold(0, |prev, x| {
            if x > &zero && number % x == 0 {
                return prev + 1;
            }
            return prev;
        });
        results.push(divisible);
        test_cases_number -= 1;
    }
    for result in results {
        println!("{}", result);
    }
}

