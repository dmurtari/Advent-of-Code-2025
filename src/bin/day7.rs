use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut manifold = Vec::<Vec<char>>::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        manifold.push(line.chars().collect());
    }

    let end_state = split_beam(manifold);
    let timelines = calculate_timelines(end_state);
    println!("{} timelines", timelines);
}

fn split_beam(manifold: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut end_state = manifold.clone();
    let mut split_count = 0;

    for row_idx in 0..end_state.len() - 1 {
        let row = end_state[row_idx].clone();
        for char_idx in 0..row.len() {
            let char = row[char_idx];
            if char == 'S' {
                end_state[row_idx + 1][char_idx] = '|';
            }

            if char == '^' && end_state[row_idx - 1][char_idx] == '|' {
                end_state[row_idx][char_idx - 1] = '|';
                end_state[row_idx][char_idx + 1] = '|';
                end_state[row_idx + 1][char_idx - 1] = '|';
                end_state[row_idx + 1][char_idx + 1] = '|';

                if end_state[row_idx - 1][char_idx - 1] != '!'
                    && end_state[row_idx - 1][char_idx + 1] != '!'
                {
                    split_count += 1;
                }
            }

            if char == '|' {
                if end_state[row_idx + 1][char_idx] == '.' {
                    end_state[row_idx + 1][char_idx] = '|';
                }
            }
        }
    }

    println!("Split {} times", split_count);
    return end_state;
}

fn calculate_timelines(manifold: Vec<Vec<char>>) -> i64 {
    let mut timelines: Vec<Vec<i64>> = vec![vec![0; manifold[0].len()]; manifold.len()];

    for row_idx in 0..manifold.len() - 1 {
        let row = manifold[row_idx].clone();
        for char_idx in 0..row.len() {
            let char = row[char_idx];
            if char == 'S' {
                timelines[row_idx][char_idx] = 1;
                continue;
            }

            if char == '|' {
                if (char_idx as isize) - 1 >= 0
                    && char_idx + 1 < row.len()
                    && manifold[row_idx][char_idx - 1] == '^'
                    && manifold[row_idx][char_idx + 1] == '^'
                {
                    timelines[row_idx][char_idx] = timelines[row_idx - 1][char_idx - 1]
                        + timelines[row_idx - 1][char_idx + 1]
                        + timelines[row_idx - 1][char_idx];
                } else if (char_idx as isize) - 1 >= 0 && manifold[row_idx][char_idx - 1] == '^' {
                    timelines[row_idx][char_idx] =
                        timelines[row_idx - 1][char_idx - 1] + timelines[row_idx - 1][char_idx];
                } else if char_idx + 1 < row.len() && manifold[row_idx][char_idx + 1] == '^' {
                    timelines[row_idx][char_idx] =
                        timelines[row_idx - 1][char_idx + 1] + timelines[row_idx - 1][char_idx];
                } else {
                    timelines[row_idx][char_idx] = timelines[row_idx - 1][char_idx];
                }

                continue;
            }
        }
    }

    return timelines[timelines.len() - 2]
        .iter()
        .fold(0, |acc, val| acc + val);
}

fn print_manifold(manifold: &Vec<Vec<char>>) {
    for line in manifold {
        println!("{:?}", line.iter().collect::<String>());
    }
}
