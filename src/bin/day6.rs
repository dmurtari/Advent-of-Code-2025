use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut rows = Vec::new();
    let mut untrimmed_rows = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        rows.push(line.split_whitespace().collect::<Vec<&str>>());
        untrimmed_rows.push(line);
    }

    let mut sum = 0;
    for n in 0..rows[0].len() {
        sum += calculate(rows.clone(), n);
    }
    println!("Found sum {}", sum);

    // #######

    let indicies = get_indicies(untrimmed_rows.last().unwrap());
    let numbers = get_ceph_numbers(untrimmed_rows, indicies);

    let mut ceph_sum = 0;
    for row in numbers {
        ceph_sum += caphalopod_calculate(row);
    }

    println!("Sum with Cephalopod Math {}", ceph_sum);
}

fn calculate(input: Vec<Vec<&str>>, col: usize) -> i64 {
    let numbers: Vec<i64> = input[..input.len() - 1]
        .iter()
        .map(|num| num[col].parse::<i64>().unwrap())
        .collect();

    let operation = input[input.len() - 1][col];

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

fn get_indicies(input: &str) -> Vec<Vec<usize>> {
    let mut indicies = Vec::new();

    let mut start = 0usize;
    let mut end;

    let chars = input.chars().collect::<Vec<char>>();

    for cursor in 0..chars.len() {
        if chars[cursor] != ' ' {
            start = cursor;
            continue;
        }

        if cursor + 1 >= chars.len() || chars[cursor + 1] != ' ' {
            end = cursor;

            if !(cursor + 1 >= chars.len()) {
                end -= 1;
            }

            indicies.push([start, end].to_vec());
            continue;
        }
    }

    return indicies;
}

fn get_ceph_numbers(input: Vec<&str>, indicies: Vec<Vec<usize>>) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = Vec::new();

    for _ in 0..indicies.len() {
        result.push(Vec::new());
    }

    for row in input {
        for (pos, index) in indicies.iter().enumerate() {
            result[pos].push(&row[index[0]..=index[1]]);
        }
    }

    return result;
}

fn caphalopod_calculate(input: Vec<&str>) -> i64 {
    let operation = input.last().unwrap().trim();

    let mut nums = Vec::new();
    for i in (0..input[0].len()).rev() {
        let num = input[0..input.len() - 1]
            .iter()
            .fold("".to_string(), |acc, cur| {
                return acc.to_string() + &cur.chars().collect::<Vec<char>>()[i].to_string();
            })
            .trim()
            .parse::<i64>()
            .unwrap();

        nums.push(num);
    }

    return nums
        .iter()
        .fold(if operation == "+" { 0 } else { 1 }, |acc, cur| {
            if operation == "+" {
                acc + cur
            } else {
                acc * cur
            }
        });
}
