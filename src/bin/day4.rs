use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut rolls = Vec::<Vec<char>>::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        rolls.push(line.chars().collect());
    }

    let mut total = 0;
    let mut iter = 0;

    let mut roll_history = Vec::<Vec<Vec<char>>>::new();
    roll_history.push(rolls.clone());

    loop {
        let mut removed = 0;
        roll_history.push(roll_history[iter].clone());

        let cur_rolls = roll_history[iter].clone();

        for i in 0..cur_rolls.len() {
            for j in 0..cur_rolls[0].len() {
                if cur_rolls[i][j] == '.' {
                    continue;
                }

                if count_neighbors(
                    cur_rolls.clone(),
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                ) < 4
                {
                    roll_history[iter + 1][i][j] = '.';
                    total += 1;
                    removed += 1;
                }
            }
        }

        iter += 1;

        if removed == 0 {
            break;
        }
    }

    println!("Total is {}", total);
}

fn count_neighbors(vec: Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut count = 0;

    for x in i - 1..=i + 1 {
        for y in j - 1..=j + 1 {
            if x >= 0
                && y >= 0
                && !(x == i && y == j)
                && x < vec[0].len().try_into().unwrap()
                && y < vec.len().try_into().unwrap()
            {
                if vec[x as usize][y as usize] == '@' {
                    count += 1
                }
            }
        }
    }

    return count;
}
