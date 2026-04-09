// https://www.hackerrank.com/challenges/migratory-birds/problem
use std::io;

fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut count = vec![0; 6];

    for bird in arr {
        count[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..=5 {
        if count[i] > max_count {
            max_count = count[i];
            result = i as i32;
        }
    }

    result
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = migratory_birds(arr);
    println!("{}", result);
}