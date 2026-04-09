// https://www.hackerrank.com/challenges/kangaroo/problem
use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = kangaroo(values[0], values[1], values[2], values[3]);
    println!("{}", result);
}