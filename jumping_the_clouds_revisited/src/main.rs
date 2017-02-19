//https://www.hackerrank.com/challenges/jumping-the-clouds-revisited

use std::io::{stdin};

fn jump(cloud: u32, distance: u32, clouds: &Vec<u32>, energy: u32) -> (u32, u32) {
    let target;
    if cloud + distance >= clouds.len() as u32 {
        target = (cloud + distance) % clouds.len() as u32;
    } else {
        target = cloud + distance;
    }
    let nrg = energy - clouds[target as usize] * 2 - 1;

    // println!("target: {:?}, energy: {:?}", target, nrg);
    
    return (target, nrg);
}

fn main() {
    let mut parameters_input: String = String::new();
    stdin().read_line(&mut parameters_input).expect("invalid input");
    let parameters = parameters_input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distance = parameters[1];

    let mut clouds_input: String = String::new();
    stdin().read_line(&mut clouds_input).expect("invalid input");
    let clouds: Vec<u32> = clouds_input.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let energy = 100;
    let mut j = jump(0, distance, &clouds, energy);

    while j.0 != 0 {
        j = jump(j.0, distance, &clouds, j.1);
    }
    println!("{:?}", j.1);
}

