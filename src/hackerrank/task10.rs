// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;
use std::io;

fn sock_merchant(ar: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut pairs = 0;

    for sock in ar {
        let count = map.entry(sock).or_insert(0);
        *count += 1;

        if *count % 2 == 0 {
            pairs += 1;
        }
    }

    pairs
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = sock_merchant(ar);
    println!("{}", result);
}