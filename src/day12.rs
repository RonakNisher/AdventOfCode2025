use itertools::Itertools;
use std::{collections::HashMap, vec};

pub fn solve(input: String) {
    let mut result: u128 = 0;

    let mut shapes: HashMap<String, i32> = HashMap::new();
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let unique_presents_count = lines.len() - 1;
    for i in 0..lines.len() - 1 {
        // shapes
        let id = lines[i]
            .lines()
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>();

        let count_blocks = lines[i].lines().skip(1).fold(0, |acc, line| {
            acc + line.chars().filter(|&c| c == '#').count() as i32
        });

        shapes.insert(id.to_string(), count_blocks);
    }

    // actual input
    for line in lines.last().unwrap().lines() {
        let (grid_size_line, presents_lines) = line.split_once(": ").unwrap();
        let (width, height) = grid_size_line
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let max_area = (width * height) as i32;
        let mut presents_needed: Vec<i32> = vec![0; unique_presents_count];
        presents_lines
            .trim()
            .split(' ')
            .enumerate()
            .for_each(|(index, present_id)| {
                presents_needed[index] = present_id.parse::<i32>().unwrap();
            });
        let mut total_shape_area = 0;
        for i in 0..presents_needed.len() {
            if presents_needed[i] > 0 {
                total_shape_area += shapes.get(&(i).to_string()).unwrap() * presents_needed[i];
            }
        }

        if total_shape_area <= max_area {
            result += 1;
        }
    }

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("*******************");
}
