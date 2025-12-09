use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::i64;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut input_boxes = Vec::new();
    let binding = fs::read_to_string(file_path).unwrap();

    for line in binding.lines() {
        input_boxes.push(line);
    }

    let mut boxes = Vec::new();
    let re = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();

    for ib in input_boxes {
        let Some(caps) = re.captures(ib) else {
            println!("Failed to parse {}", ib);
            continue;
        };

        let x = caps[1].to_string().parse::<i64>().unwrap();
        let y = caps[2].to_string().parse::<i64>().unwrap();
        let z = caps[3].to_string().parse::<i64>().unwrap();

        boxes.push([x, y, z].to_vec());
    }

    let mut distances: Vec<Vec<i64>> = vec![vec![i64::MAX; boxes.len()]; boxes.len()];
    for x in 0..boxes.len() {
        for y in 0..boxes.len() {
            if x <= y {
                continue;
            }
            distances[x][y] = distance(&boxes[x], &boxes[y]);
        }
    }

    let mut ordered_indicies = Vec::new();
    loop {
        let mut min = i64::MAX;
        let mut min_indicies = [0, 0];
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                if distances[i][j] < min {
                    min = distances[i][j];
                    min_indicies = [i, j];
                }
            }
        }

        ordered_indicies.push([min_indicies[0], min_indicies[1]]);
        distances[min_indicies[0]][min_indicies[1]] = i64::MAX;

        if ordered_indicies.len() == boxes.len() {
            break;
        }
    }

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..1000 {
        let connection = ordered_indicies[i];
        graph
            .entry(connection[0])
            .and_modify(|c| c.push(connection[1]))
            .or_insert([connection[1]].to_vec());
        graph
            .entry(connection[1])
            .and_modify(|c| c.push(connection[0]))
            .or_insert([connection[0]].to_vec());
    }

    let mut circuits = Vec::new();
    let mut deduped = Vec::new();

    for (node, _edges) in &graph {
        circuits.push(bfs(&graph, *node));
    }

    deduped.push(&circuits[0]);
    'outer: for candidate in &circuits {
        for c in 0..deduped.len() {
            if are_sets_equal(&candidate, &deduped[c]) {
                println!("Same");
                continue 'outer;
            }
        }
        deduped.push(&candidate);
    }

    let mut sizes = deduped.iter().fold(Vec::new(), |mut acc, c| {
        acc.push(c.len());
        return acc;
    });

    sizes.sort();
    sizes.reverse();

    let mut product = 1;
    for i in 0..3 {
        product *= sizes[i];
    }

    println!("Sizes {:?}", sizes);
    println!("Product {}", product);
}

fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> HashSet<usize> {
    let mut circuit = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front(start);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if !circuit.contains(&node) {
            circuit.insert(node);
            let maybe_neighbors = graph.get(&node);

            match maybe_neighbors {
                None => continue,
                Some(neighbors) => {
                    for neighbor in neighbors {
                        if !circuit.contains(&neighbor) {
                            queue.push_back(*neighbor);
                        }
                    }
                }
            }
        }
    }

    return circuit;
}

fn are_sets_equal(set1: &HashSet<usize>, set2: &HashSet<usize>) -> bool {
    println!("Compparing {:?}, {:?}", set1, set2);
    if set1.len() != set2.len() {
        return false;
    }

    for element in set1 {
        if !set2.contains(element) {
            return false;
        }
    }

    return true;
}

fn distance(box1: &Vec<i64>, box2: &Vec<i64>) -> i64 {
    return ((box1[0] - box2[0]).pow(2) + (box1[1] - box2[1]).pow(2) + (box1[2] - box2[2]).pow(2))
        .isqrt();
}
