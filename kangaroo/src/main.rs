//https://www.hackerrank.com/challenges/kangaroo
use std::io::{stdin};

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("invalid input");
    let data:Vec<i32> = input.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let x1 = data[0];
    let v1 = data[1];
    let x2 = data[2];
    let v2 = data[3];

    //get behind kangaroo
    if x1 == x2 {
        if v1 == v2 {
            println!("{}", "YES");
        } else {
            println!("{}", "NO");
        }
    }
    let mut back;
    let mut front;
    let back_step;
    let front_step;
    if x1 < x2 {
        back = x1;
        back_step = v1;
        front = x2;
        front_step = v2;
    } else {
        front = x1;
        front_step = v1;
        back = x2;
        back_step = v2;
    }

    if back_step < front_step {
        println!("{}", "NO");   
    } else {
        let mut success = false;
        //start jumping until the one back gets in front
        while back < front {
            back += back_step;
            front += front_step;

            success = back == front 
        }
        println!("{}", if success { "YES" } else { "NO" });  
    }
}
