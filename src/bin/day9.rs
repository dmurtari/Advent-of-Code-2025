use std::env;
use std::fs;
use std::i64;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut input_coords = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        input_coords.push(line);
    }

    let mut coordinates = Vec::new();
    let re = Regex::new(r"^(\d+),(\d+)$").unwrap();

    for ib in input_coords {
        let Some(caps) = re.captures(ib) else {
            println!("Failed to parse {}", ib);
            continue;
        };

        let x = caps[1].to_string().parse::<i64>().unwrap();
        let y = caps[2].to_string().parse::<i64>().unwrap();

        coordinates.push([x, y].to_vec());
    }

    println!("Parsed input");

    let max_area = find_largest_area(&coordinates);

    println!("Largest area {}", max_area);
}

fn find_largest_area(coords: &Vec<Vec<i64>>) -> i64 {
    let mut max_found = 0;

    for i in 0..coords.len() {
        for j in 0..coords.len() {
            let local_area = area(&coords[i], &coords[j]);

            if local_area > max_found {
                max_found = local_area;
            }
        }
    }

    return max_found;
}

fn area(coord_1: &Vec<i64>, coord_2: &Vec<i64>) -> i64 {
    let area = (coord_1[0] - coord_2[0] + 1).abs() * (coord_1[1] - coord_2[1] + 1).abs();
    println!("Area between {:?} and {:?} is {}", coord_1, coord_2, area);
    return area;
}
