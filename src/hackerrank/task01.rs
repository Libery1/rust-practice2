// https://www.hackerrank.com/challenges/birthday-cake-candles
use std::io::{self, BufRead};

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_val = match candles.iter().max() {
        Some(&max) => max,
        None => return 0,
    };

    candles.iter().filter(|&&x| x == max_val).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let _n = iterator.next().unwrap().unwrap();

    let candles_line = iterator.next().unwrap().unwrap();
    
    let candles: Vec<i32> = candles_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);

    println!("{}", result);
}