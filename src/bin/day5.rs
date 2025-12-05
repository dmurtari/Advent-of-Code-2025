use regex::Regex;
use std::cmp;
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
        let as_u64 = id.to_string().parse::<i64>().unwrap();

        for range in &valid_id_ranges {
            if as_u64 >= range[0] && as_u64 <= range[1] {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("Total fresh: {}", fresh_count);

    let all_valid_ranges = generate_all_valid(valid_ids);
    let sum_of_range_length = all_valid_ranges.iter().fold(0, |acc, cur| {
        return acc + (cur[1] - cur[0] + 1);
    });

    println!("Total valid IDs: {}", sum_of_range_length);
}

fn parse_valid_ranges(ranges: &Vec<&str>) -> Vec<Vec<i64>> {
    let mut result = Vec::new();

    let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    for range in ranges {
        let Some(caps) = re.captures(range) else {
            continue;
        };

        let start = caps[1].to_string().parse::<i64>().unwrap();
        let end = caps[2].to_string().parse::<i64>().unwrap();
        result.push([start, end].to_vec());
    }

    return result;
}

fn generate_all_valid(ranges: Vec<&str>) -> Vec<Vec<i64>> {
    let parsed = parse_valid_ranges(&ranges);
    let mut old_parsed = parsed.clone();

    loop {
        let mut did_simplify = false;
        let mut new_parsed: Vec<Vec<i64>> = Vec::new();

        'outer: for pair in &old_parsed {
            if new_parsed.len() == 0 {
                new_parsed.push(pair.to_vec());
                continue;
            }

            for existing in new_parsed.iter_mut() {
                if cmp::max::<i64>(existing[0], pair[0]) - cmp::min::<i64>(existing[1], pair[1])
                    <= 0
                {
                    existing[0] = cmp::min(existing[0], pair[0]);
                    existing[1] = cmp::max(existing[1], pair[1]);
                    did_simplify = true;
                    continue 'outer;
                }
            }

            new_parsed.push(pair.to_vec());
        }

        if !did_simplify {
            break;
        }

        old_parsed = new_parsed;
    }

    return old_parsed;
}
