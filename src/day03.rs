use std::fs;
use std::io::{self, BufRead};

use regex::Regex;

pub fn resolve() {
    let occurrences = get_occurrences();
    let sum = sum_occurrences(occurrences);

    println!("{}", sum);
}

fn get_occurrences() -> Vec<(i64, i64)> {
    let file = fs::File::open("inputs/day03.txt").expect("Should be able to open the file");
    let reader = io::BufReader::new(file);
    let mul = Regex::new(r"(mul)\((\d+),(\d+)\)").unwrap();
    let dont = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let mut content = String::new();

    for line in reader.lines() {
        match line {
            Ok(l) => content.push_str(l.as_str()),
            Err(err) => eprintln!("Shouldn't have a error: {}", err),
        }
    }

    let removed_expr = dont.replace_all(content.as_str(), "").into_owned();
    let results: Vec<(i64, i64)> = mul
        .find_iter(removed_expr.as_str())
        .map(|m| m.as_str())
        .filter_map(|s| {
            mul.captures(s).map(|caps| {
                let first = caps[2].parse::<i64>().expect("Should be a number");
                let second = caps[3].parse::<i64>().expect("Should be a number");
                (first, second)
            })
        })
        .collect();

    results
}

fn sum_occurrences(occurrences: Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;

    for (value1, value2) in occurrences {
        sum += value1 * value2;
    }

    sum
}
