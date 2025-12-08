use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Box {
    x: i32,
    y: i32,
    z: i32,
}

fn get_euclidean_distance(a: &Box, b: &Box) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn calculate(euclidean_distances: &Vec<(f64, (Box, Box))>, is_part_1: bool) -> i32 {
    let mut result: i32 = 0;
    let mut circuits: Vec<HashSet<Box>> = vec![];
    let mut boxes = HashSet::new();
    euclidean_distances.iter().for_each(|&(_, (box_a, box_b))| {
        boxes.insert(box_a);
        boxes.insert(box_b);
    });

    let boxes_count = boxes.len();

    let mut distances: VecDeque<(Box, Box)> = euclidean_distances
        .iter()
        .map(|&(_, (box_a, box_b))| (box_a, box_b))
        .collect();

    loop {
        let mut found_connection = false;
        let mut found_a = false;
        let mut index_a = 0;
        let mut found_b = false;
        let mut index_b = 0;

        let (box_a, box_b) = distances.pop_front().unwrap();
        for i in 0..circuits.len() {
            if circuits[i].contains(&box_a) {
                found_a = true;
                index_a = i;
            }

            if circuits[i].contains(&box_b) {
                found_b = true;
                index_b = i;
            }

            if found_a && found_b {
                break;
            }
        }

        if found_a && found_b && index_a != index_b {
            // merge the two circuits
            let to_merge = circuits[index_b].clone();
            for b in to_merge {
                circuits[index_a].insert(b);
            }
            circuits.remove(index_b);
            found_connection = true;
        } else if found_a && found_b && index_a == index_b {
            // both boxes already in circuits, do nothing
            found_connection = true;
        } else if found_a {
            circuits[index_a].insert(box_b);
            found_connection = true;
        } else if found_b {
            circuits[index_b].insert(box_a);
            found_connection = true;
        }

        if found_connection {
            boxes.remove(&box_a);
            boxes.remove(&box_b);
        }

        if !found_connection {
            circuits.push(HashSet::from([box_a, box_b]));
        }

        if is_part_1 && distances.len() == 0 {
            break;
        }

        if !is_part_1 && circuits.len() == 1 && circuits[0].len() == boxes_count {
            result = box_a.x * box_b.x;
            break;
        }
    }

    if is_part_1 {
        circuits.sort_by(|a, b| b.len().cmp(&a.len()));
        result = circuits[0].len() as i32 * circuits[1].len() as i32 * circuits[2].len() as i32;
    }

    result
}

pub fn solve(input: String) {
    let result: i32;
    let result_part2: i32;

    let mut boxes: Vec<Box> = vec![];
    input.lines().for_each(|line| {
        let coords = line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();
        boxes.push(Box {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    });

    let mut euclidean_distances: Vec<(f64, (Box, Box))> = vec![];

    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let dist = get_euclidean_distance(&boxes[i], &boxes[j]);
            euclidean_distances.push((dist, (boxes[i], boxes[j])));
        }
    }

    euclidean_distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let part_1_input = euclidean_distances[0..1000].to_vec();
    result = calculate(&part_1_input, true /*is_part_one*/) as i32;
    result_part2 = calculate(&euclidean_distances, false /*is_part_one*/);

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
