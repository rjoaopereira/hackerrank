//https://www.hackerrank.com/challenges/beautiful-days-at-the-movies

use std::io::{stdin};

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("invalid input");
    let parameters = input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let i = parameters[0];
    let j = parameters[1];
    let k = parameters[2];

    let mut days = 0;
    for day in i..j+1 {
        let day_string = day.to_string();
        let mut reversed = day_string.split("").collect::<Vec<&str>>();
        reversed.reverse();
        let reversed_day = reversed.join("").parse::<i64>().unwrap();
        if (day - reversed_day).abs() % k == 0 {
            days += 1;
        } 
    }
    println!("{:?}", days);
}
