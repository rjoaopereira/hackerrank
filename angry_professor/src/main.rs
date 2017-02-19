//https://www.hackerrank.com/challenges/angry_professor

use std::io::{stdin};

fn main() {
    let mut test_cases_number_input: String = String::new();
    stdin().read_line(&mut test_cases_number_input).expect("invalid input");
    let mut test_cases_number = test_cases_number_input.trim().parse::<u32>().unwrap_or(0);
    let mut results: Vec<&str> = Vec::new();
    while test_cases_number > 0 {
        let mut parameters_input: String = String::new();
        stdin().read_line(&mut parameters_input).expect("invalid input");
        let parameters: Vec<i32> = parameters_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let threshold = parameters[1];
        test_cases_number -= 1; 
        let mut arrivals_input: String = String::new();
        stdin().read_line(&mut arrivals_input).expect("invalid input");
        let arrivals: Vec<i32> = arrivals_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let tardy: i32 = 0;
        let on_time = arrivals.iter().fold(0, |prev, arrival| {
            if arrival <= &tardy {
                return prev + 1;
            }
            return prev;
        });
        results.push(if on_time >= threshold { "NO" } else { "YES" } );
    }
    for result in results {
        println!("{}", result);
    }
}
