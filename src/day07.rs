use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn traverse(
    current_row: usize,
    current_col: usize,
    grid: &mut Vec<Vec<char>>,
    splitters: &mut HashSet<(usize, usize)>,
) {
    if current_col >= grid[0].len() || current_row >= grid.len() {
        return;
    }

    let current_cell = grid[current_row][current_col];

    if current_cell == '|' {
        // already visited
        return;
    }

    if current_cell == '.' {
        // traverse below
        grid[current_row][current_col] = '|';
        traverse(current_row + 1, current_col, grid, splitters);
    }

    if current_cell == '^' {
        // found a splitter
        splitters.insert((current_row, current_col));
        traverse(current_row, current_col - 1, grid, splitters);
        traverse(current_row, current_col + 1, grid, splitters);
    }
}

fn traverse_part2(
    current_row: usize,
    current_col: usize,
    grid: &mut Vec<Vec<char>>,
    visited_timelines_map: &mut HashMap<(usize, usize), u128>,
) -> u128 {
    if visited_timelines_map.contains_key(&(current_row, current_col)) {
        // already visited
        return *visited_timelines_map
            .get(&(current_row, current_col))
            .unwrap();
    }

    if current_row == grid.len() - 1 {
        visited_timelines_map
            .entry((current_row, current_col))
            .or_insert(1);
        return 1;
    }

    if current_col >= grid[0].len() || current_row >= grid.len() {
        return 0;
    }

    let current_cell = grid[current_row][current_col];

    if current_cell == '|' {
        // already visited
        return 0;
    }

    let mut timelines = 0;

    if current_cell == '.' {
        // traverse below
        grid[current_row][current_col] = '|';
        let result = traverse_part2(current_row + 1, current_col, grid, visited_timelines_map);

        visited_timelines_map
            .entry((current_row, current_col))
            .or_insert(result);

        timelines += result;
    } else if current_cell == '^' {
        // found a splitter
        let mut new_grid_1 = grid.clone();
        let mut new_grid_2 = grid.clone();
        timelines += traverse_part2(
            current_row,
            current_col - 1,
            &mut new_grid_1,
            visited_timelines_map,
        );
        timelines += traverse_part2(
            current_row,
            current_col + 1,
            &mut new_grid_2,
            visited_timelines_map,
        );

        visited_timelines_map
            .entry((current_row, current_col))
            .or_insert(timelines);
    }

    return timelines;
}

pub fn solve(input: String) {
    let result: i32;
    let result_part2: u128;

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start_row = 1;
    let start_col = grid[0].iter().find_position(|&c| *c == 'S').unwrap().0;

    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    let mut grid_part1 = grid.clone();
    traverse(start_row, start_col, &mut grid_part1, &mut splitters);
    result = splitters.len() as i32;

    //////////
    // Part 2
    //////////
    let mut visited_timelines_map: HashMap<(usize, usize), u128> = HashMap::new();
    result_part2 = traverse_part2(start_row, start_col, &mut grid, &mut visited_timelines_map);

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
