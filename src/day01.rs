use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Error};

pub fn resolve() {
    let (mut list_one, mut list_two) = read_file("inputs/day01.txt");

    list_one.sort();
    list_two.sort();

    let distance = get_distance(&mut list_one, &mut list_two);
    let score = get_score(list_one, list_two);

    println!("Total distance: {}", distance);
    println!("Score: {}", score);
}

fn read_file(path: &str) -> (Vec<i64>, Vec<i64>) {
    let file = fs::File::open(path).expect("Should be able to open the file");
    let reader = io::BufReader::new(file);
    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();

    for line in reader.lines() {
        match line_tuple_to_numbers(line) {
            Ok((n1, n2)) => {
                list_one.push(n1);
                list_two.push(n2);
            }
            Err(err) => eprintln!("{}", err),
        }
    }

    (list_one, list_two)
}

fn line_tuple_to_numbers(line: Result<String, Error>) -> Result<(i64, i64), String> {
    match line {
        Ok(content) => {
            let mut numbers = content.split_whitespace();
            let number_one = numbers
                .next()
                .ok_or("Missing first number".to_string())?
                .parse::<i64>()
                .map_err(|_| "Failed to parse this number".to_string())?;

            let number_two = numbers
                .next()
                .ok_or("Missing first number".to_string())?
                .parse::<i64>()
                .map_err(|_| "Failed to parse this number".to_string())?;

            Ok((number_one, number_two))
        }
        Err(err) => Err(format!("Error reading a line: {}", err)),
    }
}

fn get_distance(list_one: &mut Vec<i64>, list_two: &mut Vec<i64>) -> i64 {
    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn get_score(list_one: Vec<i64>, list_two: Vec<i64>) -> i64 {
    let mut hash = HashMap::new();

    for number in list_one {
        hash.entry(number)
            .and_modify(|(count, _)| *count += 1)
            .or_insert((1, 0));
    }

    for number in list_two {
        hash.entry(number).and_modify(|(_, value)| *value += 1);
    }

    hash.into_iter()
        .map(|(number, (count, value))| number * count * value)
        .sum()
}
