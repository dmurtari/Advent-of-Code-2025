use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut rows = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        rows.push(line.split_whitespace().collect::<Vec<&str>>());
    }

    let mut sum = 0;
    for n in 0..rows[0].len() {
        sum += calculate(rows.clone(), n);
    }

    println!("Found sum {}", sum);
}

fn calculate(input: Vec<Vec<&str>>, col: usize) -> i64 {
    let numbers: Vec<i64> = input[..input.len() - 1]
        .iter()
        .map(|num| num[col].parse::<i64>().unwrap())
        .collect();

    let operation = input[input.len() - 1][col];

    println!("Performing {} on {:?}", operation, numbers);

    return numbers
        .iter()
        .fold(if operation == "+" { 0 } else { 1 }, |acc, cur| {
            if operation == "+" {
                acc + cur
            } else {
                acc * cur
            }
        });
}
