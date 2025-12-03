use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let method = &args[1];
    let file_path = &args[2];

    let mut lines = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        lines.push(line);
    }

    let dial_size = 100;
    let mut dial = 50;
    let mut result = 0;
    let re = Regex::new(r"^([L|R])(\d{1,})$").unwrap();

    for line in lines {
        let Some(caps) = re.captures(line) else {
            println!("Failed to parse {}", line);
            return;
        };

        println!("Line is {}", line);
        println!("Dial is {}", dial);

        let direction = &caps[1];
        let count = caps[2].to_string().parse::<i32>().unwrap();

        let mut passed = 0;
        if direction == "R" {
            let mut target = dial + count;

            if target % dial_size == 0 {
                target = target - dial_size;
            }

            while target > dial_size - 1 {
                target = target - dial_size;
                passed = passed + 1;
            }

            dial = target;
        } else if direction == "L" {
            let mut target = dial - count;

            if dial == 0 {
                target = target + dial_size;
            }

            while target < 0 {
                target = target + dial_size;
                passed = passed + 1;
            }

            dial = target;
        }

        if method == "hex" {
            println!("Passed {} times", passed);
            result = result + passed;
        }

        println!("Dial is {}", dial);
        if dial == 0 {
            result = result + 1;
        }

        println!("Result is {}", result);
        println!("")
    }

    println!("Password is {}", result)
}
