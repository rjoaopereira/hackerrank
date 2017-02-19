//https://www.hackerrank.com/challenges/between-two-sets
use std::io::{stdin};

fn vector_has_factors_of_step(step:&i32, set: &Vec<i32>) -> bool {
    // println!("1 - step: {},  set: {:?}", step, set);
    set.iter().all(|&x| step % x == 0)
}

fn step_is_factor_of_set(step:&i32, set: &Vec<i32>) -> bool {
    // println!("2 - step: {},  set: {:?}", step, set);
    set.iter().all(|&x| x % step == 0)
}

fn main() {
    let mut size_input: String = String::new();
    stdin().read_line(&mut size_input).expect("invalid input");
    let size:Vec<i32> = size_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut first_set_input: String = String::new();
    stdin().read_line(&mut first_set_input).expect("invalid input");
    let mut first_set:Vec<i32> = first_set_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    first_set.sort();
    let mut second_set_input: String = String::new();
    stdin().read_line(&mut second_set_input).expect("invalid input");
    let mut second_set:Vec<i32> = second_set_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    second_set.sort();  
    let step: i32 = first_set[size[0] as usize - 1];
    let mut path:i32 = step;
    let mut count = 0;

    while path <= second_set[size[1] as usize - 1] {
        if vector_has_factors_of_step(&path, &first_set) && step_is_factor_of_set(&path, &second_set) {
            count += 1;
        }
        path += step;
    }

    println!("{}", count);
}
