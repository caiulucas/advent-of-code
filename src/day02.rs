use std::fs;
use std::io::{self, BufRead};

pub fn resolve() {
    let safe_count = read_file("inputs/day02.txt");
    println!("{}", safe_count);
}

fn read_file(path: &str) -> i32 {
    let file = fs::File::open(path).expect("Should be able to open the file");
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let numbers = match line {
            Ok(content) => content
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("Should be a number"))
                .collect(),
            _ => Vec::new(),
        };

        if verify_safety(numbers) {
            safe_count += 1;
        }
    }

    safe_count
}

fn verify_safety(numbers: Vec<i32>) -> bool {
    if is_safe(&numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut aux = numbers.clone();
        aux.remove(i);

        if is_safe(&aux) {
            return true;
        }
    }

    false
}

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let is_increasing = numbers[0] < numbers[1];
    for i in 1..numbers.len() {
        if is_increasing && numbers[i - 1] >= numbers[i]
            || !is_increasing && numbers[i - 1] <= numbers[i]
            || (numbers[i - 1] - numbers[i]).abs() > 3
        {
            return false;
        }
    }

    true
}
