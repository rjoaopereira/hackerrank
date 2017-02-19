//https://www.hackerrank.com/challenges/circular-array-rotation
use std::io::{stdin};

fn convert(v: &str) -> Vec<i32>{
    v.split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn main() {
    let n;
    let mut k;
    let mut q;

    let mut first_input: String = String::new();
    let mut second_input: String = String::new();

    stdin().read_line(&mut first_input).expect("Did not enter a correct string");
    stdin().read_line(&mut second_input).expect("Did not enter a correct string");

    // n k q
    let first_input_vec: Vec<i32> = convert(&first_input);
    //sample (1 2 3 4)
    let mut sample: Vec<i32> = convert(&second_input);


    n = first_input_vec[0];// size of sample
    k = first_input_vec[1];// number of rotations
    q = first_input_vec[2];// number of queries
    //reset vector length
    sample.resize(n as usize, 0);

    let mut last;
    //rotate the vector k times
    while k > 0 {
        k -=1;
        //swap is probably more efficient
        last = sample.pop().unwrap();
        sample.insert(0, last);
    }

    let mut position = String::new();
    while q > 0 {
        q -= 1;
        position.clear();
        stdin().read_line(&mut position).expect("Did not enter a correct string");
        println!("{}", sample[position.trim().parse::<i32>().unwrap() as usize]);
    }
}
