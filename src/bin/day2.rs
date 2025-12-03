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
        let Some(caps) = re.captures(range) else {
            println!("Failed to parse {}", range);
            return;
        };

        let mut inner_count = 0;
        let start = caps[1].to_string().parse::<i64>().unwrap();
        let end = caps[2].to_string().parse::<i64>().unwrap();

        for n in start..end + 1 {
            let count = repeat_count(n);

            if count > 0 {
                inner_count += count;
                sum += n
            }
        }

        println!("{}: {}", range, inner_count);
    }

    println!("Total: {}", sum);
}

fn repeat_count(num: i64) -> i64 {
    let num_as_str = num.to_string();
    let str_len = num_as_str.len();
    let num_cursors = str_len;

    let mut count = 0;

    for n in 2..num_cursors + 1 {
        if str_len % n != 0 {
            continue;
        }

        let mut sub_str = Vec::new();
        let hop = str_len / n;
        let mut cursor = 0;

        while cursor < str_len {
            sub_str.push(&num_as_str[cursor..cursor + hop]);
            cursor += hop;
        }

        if sub_str.len() == 0 {
            continue;
        }

        let is_all_eq = sub_str
            .iter()
            .fold(true, |acc, &str| acc && str == sub_str[0]);

        if is_all_eq {
            count += 1;
        }
    }

    return count;
}

fn _is_repeated_twice(num: i64) -> bool {
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
