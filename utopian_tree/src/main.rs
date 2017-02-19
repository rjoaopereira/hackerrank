//https://www.hackerrank.com/challenges/utopian_tree

use std::io::{stdin};

fn main() {
    let mut test_cases_number_input: String = String::new();
    stdin().read_line(&mut test_cases_number_input).expect("invalid input");
    let mut test_cases_number = test_cases_number_input.trim().parse::<u32>().unwrap_or(0);
    let mut results: Vec<u32> = Vec::new();
    while test_cases_number > 0 {
        let mut cycles_input: String = String::new();
        stdin().read_line(&mut cycles_input).expect("invalid input");
        let cycles = cycles_input.trim().parse::<u32>().unwrap();
        test_cases_number -= 1; 
        let mut height = 1;
        for i in 0..cycles {
            if i % 2 != 0 {
                height += 1;
            } else {
                height *= 2;
            }
        }
        results.push(height);
    }
    for result in results {
        println!("{:?}", result);
    }
}
