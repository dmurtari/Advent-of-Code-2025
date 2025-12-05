use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut valid_ids = Vec::new();
    let mut test_ids = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    let mut is_parsing_ids = true;
    for line in binding.lines() {
        if line == "" {
            is_parsing_ids = false;
            continue;
        }

        if is_parsing_ids {
            valid_ids.push(line);
        } else {
            test_ids.push(line);
        }
    }

    let valid_id_ranges = parse_valid_ranges(&valid_ids);
    let mut fresh_count = 0;

    for id in test_ids {
        let as_u64 = id.to_string().parse::<u64>().unwrap();
        println!(".");

        for range in &valid_id_ranges {
            if as_u64 >= range[0] && as_u64 <= range[1] {
                fresh_count += 1;
                println!("{} is valid", as_u64);
                break;
            }
        }
    }

    println!("Total fresh: {}", fresh_count);
}

fn parse_valid_ranges(ranges: &Vec<&str>) -> Vec<Vec<u64>> {
    let mut result = Vec::new();

    let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    for range in ranges {
        println!("Range {}", range);
        let Some(caps) = re.captures(range) else {
            println!("Failed to parse {}", range);
            continue;
        };

        let start = caps[1].to_string().parse::<u64>().unwrap();
        let end = caps[2].to_string().parse::<u64>().unwrap();
        result.push([start, end].to_vec());
    }

    return result;
}
