use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut ranges = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for range in binding.split(",") {
        ranges.push(range.trim());
    }

    let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let mut sum = 0;

    for range in ranges {
        println!("Range is {}", range);
        let Some(caps) = re.captures(range) else {
            println!("Failed to parse {}", range);
            return;
        };

        let mut inner_count = 0;
        let start = caps[1].to_string().parse::<i64>().unwrap();
        let end = caps[2].to_string().parse::<i64>().unwrap();

        for n in start..end + 1 {
            if is_repeating(n) {
                inner_count += 1;
                sum += n;
            }
        }

        println!("Found {} in range {}", inner_count, range);
    }

    println!("Total: {}", sum);
}

fn is_repeating(num: i64) -> bool {
    let num_as_str = num.to_string();

    if num_as_str.len() % 2 != 0 {
        return false;
    }

    let mut cursor_1 = 0;
    let mut cursor_2 = num_as_str.len() / 2;

    for _ in 0..num_as_str.len() / 2 {
        if num_as_str.chars().nth(cursor_1) != num_as_str.chars().nth(cursor_2) {
            return false;
        }

        cursor_1 += 1;
        cursor_2 += 1;
    }

    return true;
}
