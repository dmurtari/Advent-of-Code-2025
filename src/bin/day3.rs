use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut banks = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        banks.push(line);
    }

    let sum = banks
        .iter()
        .fold(0, |acc, bank| acc + get_supermax_joltage(bank));

    println!("Total output Joltage is: {}", sum)
}

fn get_supermax_joltage(bank: &str) -> i64 {
    let mut parsed_bank = Vec::new();

    for char in bank.chars() {
        parsed_bank.push(char.to_string().parse::<i32>().unwrap());
    }

    let mut digits = Vec::new();
    let mut prev_pos = 0;

    for n in 0..12 {
        let padding = 12 - n;
        let section = &parsed_bank[prev_pos..parsed_bank.len() - padding + 1];
        let d = section.iter().max().unwrap();
        let pos = section.iter().position(|x| x == d).unwrap() + prev_pos;

        prev_pos = pos + 1;
        digits.push(d);
    }

    let result = digits
        .iter()
        .map(|x| x.to_string())
        .reduce(|cur, next| cur + &next)
        .unwrap();

    println!("Result is {}", result);
    return result.to_string().parse::<i64>().unwrap();
}

fn get_max_joltage(bank: &str) -> i32 {
    let mut parsed_bank = Vec::new();

    for char in bank.chars() {
        parsed_bank.push(char.to_string().parse::<i32>().unwrap());
    }

    let d1 = parsed_bank[0..parsed_bank.len() - 1].iter().max().unwrap();
    let d1_pos = parsed_bank[0..parsed_bank.len() - 1]
        .iter()
        .position(|x| x == d1)
        .unwrap();

    let d2 = parsed_bank[d1_pos + 1..parsed_bank.len()]
        .iter()
        .max()
        .unwrap();

    let result = format!("{}{}", d1, d2);
    println!("Max for bank {} is: {}", bank, result);
    return result.to_string().parse::<i32>().unwrap();
}
