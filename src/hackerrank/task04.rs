// https://www.hackerrank.com/challenges/grading/problem
use std::io;

fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;
                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut grades = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        grades.push(input.trim().parse::<i32>().unwrap());
    }

    let result = grading_students(grades);

    for grade in result {
        println!("{}", grade);
    }
}