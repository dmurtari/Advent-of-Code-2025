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

    split_beam(manifold);
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

    print_manifold(&end_state);
    println!("Split {} times", split_count);
    return end_state;
}

fn print_manifold(manifold: &Vec<Vec<char>>) {
    for line in manifold {
        println!("{:?}", line.iter().collect::<String>());
    }
}
