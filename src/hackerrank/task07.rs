// https://www.hackerrank.com/challenges/between-two-sets/problem
use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut lcm_a = a[0];
    for &num in &a {
        lcm_a = lcm(lcm_a, num);
    }

    let mut gcd_b = b[0];
    for &num in &b {
        gcd_b = gcd(gcd_b, num);
    }

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = get_total_x(a, b);
    println!("{}", result);
}