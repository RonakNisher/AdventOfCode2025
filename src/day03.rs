use itertools::Itertools;
use itertools::rev;

fn get_n_largest_batteries(batteries: &Vec<u32>, n: usize) -> u128 {
    let mut joltage = String::from("");
    let len_batteries = batteries.len();
    let mut prev_lower_index = 0;
    for i in rev(0..n) {
        let max_limit = len_batteries - 1 - i;

        let mut max_value = 0;
        let mut max_position = 0;
        for j in prev_lower_index..max_limit + 1 {
            if batteries[j] > max_value {
                max_value = batteries[j];
                max_position = j;
            }
        }

        joltage += &max_value.to_string();
        prev_lower_index = max_position + 1;
    }

    joltage.parse::<u128>().unwrap()
}

pub fn solve(input: String) {
    let mut result: u128 = 0;
    let mut result_part2: u128 = 0;

    input.lines().for_each(|line| {
        let batteries = line.chars().map(|x| x.to_digit(10).unwrap()).collect_vec();

        let joltage_value_part1 = get_n_largest_batteries(&batteries, 2);
        let joltage_value_part2 = get_n_largest_batteries(&batteries, 12);

        result += joltage_value_part1;
        result_part2 += joltage_value_part2;
    });

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}
