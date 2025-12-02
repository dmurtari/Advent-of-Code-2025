use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[2];

    let mut lines = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        lines.push(line);
    }

    let DIAL_SIZE: i32 = 100;
    let mut dial: i32 = 50;
    let mut result: i32 = 0;
    let re = Regex::new(r"^([L|R])(\d{1,3})$").unwrap();
    for line in lines {
        let Some(caps) = re.captures(line) else {
            println!("Failed to parse {}", line);
            return;
        };

        let direction = &caps[1];
        let count = caps[2].to_string().parse::<i32>().unwrap();

        if dial == 0 {
            result = result + 1i32;
        }

        if direction == "R" {
            dial = (dial + count) % DIAL_SIZE;
        } else if direction == "L" {
            dial = (dial - count) % DIAL_SIZE;
        }

        println!("Rotates {} to {}", line, dial)
    }

    println!("Password is {}", result)
}
