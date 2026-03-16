https://www.hackerrank.com/challenges/diagonal-difference/problem
use std::io::{self, BufRead};


fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next()
        .unwrap().unwrap()
        .trim()
        .parse().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        let row: Vec<i32> = lines.next()
            .unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(&matrix);

    println!("{}", result);
}