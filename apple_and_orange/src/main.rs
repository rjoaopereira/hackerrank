//https://www.hackerrank.com/challenges/apple-and-orange
use std::io::{stdin};

fn main() {
    let mut house_input: String = String::new();
    stdin().read_line(&mut house_input).expect("invalid input");
    let house_measures:Vec<&str> = house_input.trim().split_whitespace().collect::<Vec<&str>>();
    let s = house_measures[0].parse::<i32>().unwrap();
    let t = house_measures[1].parse::<i32>().unwrap();

    let mut trees_input: String = String::new();
    stdin().read_line(&mut trees_input).expect("invalid input");
    let trees_position:Vec<&str> = trees_input.trim().split_whitespace().collect::<Vec<&str>>();
    let a = trees_position[0].parse::<i32>().unwrap();
    let b = trees_position[1].parse::<i32>().unwrap();

    let mut fruits_input: String = String::new();
    stdin().read_line(&mut fruits_input).expect("invalid input");
    let fruit_quantity:Vec<&str> = fruits_input.trim().split_whitespace().collect::<Vec<&str>>();
    let m = fruit_quantity[0].parse::<usize>().unwrap();
    let n = fruit_quantity[1].parse::<usize>().unwrap();

    let mut apples_input: String = String::new();
    stdin().read_line(&mut apples_input).expect("invalid input");
    let apples_positions:Vec<i32> = apples_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut oranges_input: String = String::new();
    stdin().read_line(&mut oranges_input).expect("invalid input");
    let oranges_positions:Vec<i32> = oranges_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let apples: i32 = apples_positions.iter().fold(0, |mut acc, &x| {
        if a + x >= s && a + x <= t {
            acc += 1;
        }
        acc
    });
    let oranges: i32 = oranges_positions.iter().fold(0, |mut acc, &x| {
        if b + x >= s && b + x <= t {
            acc += 1;
        }
        acc
    });

    println!("{}", apples);
    println!("{}", oranges);
}
