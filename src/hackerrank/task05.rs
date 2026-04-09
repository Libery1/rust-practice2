// https://www.hackerrank.com/challenges/apple-and-orange/problem
use std::io;

fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for d in apples {
        let position = a + d;
        if position >= s && position <= t {
            apple_count += 1;
        }
    }

    for d in oranges {
        let position = b + d;
        if position >= s && position <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let mut input = String::new();

    // s t
    io::stdin().read_line(&mut input).unwrap();
    let st: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (s, t) = (st[0], st[1]);

    input.clear();

    // a b
    io::stdin().read_line(&mut input).unwrap();
    let ab: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (a, b) = (ab[0], ab[1]);

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let apples: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, apples, oranges);
}