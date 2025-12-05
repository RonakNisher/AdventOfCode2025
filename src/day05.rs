use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: String) {
    let mut result: u64 = 0;
    let mut result_part2: u64 = 0;

    let (input_a, input_b) = input.split_once("\n\n").unwrap();

    let mut fresh_ingredients_set: HashSet<(u64, u64)> = HashSet::new();
    input_a.lines().for_each(|line| {
        let (lower, upper) = line.split_once("-").unwrap();
        let lower_int = lower.parse::<u64>().unwrap();
        let upper_int = upper.parse::<u64>().unwrap();
        fresh_ingredients_set.insert((lower_int, upper_int));
    });

    input_b.lines().for_each(|line| {
        let quantity = line.parse::<u64>().unwrap();
        if fresh_ingredients_set.iter().any(|(lower, upper)| {
            if quantity >= *lower && quantity <= *upper {
                return true;
            }
            false
        }) {
            result += 1;
        }
    });

    //////////////
    // part 2
    // ///////////
    let mut fresh_ingredients_vec: Vec<(u64, u64)> =
        fresh_ingredients_set.iter().copied().sorted().collect();

    for i in 0..fresh_ingredients_vec.len() {
        let (mut lower, mut upper) = fresh_ingredients_vec[i];

        // Lower
        let mut current = lower;
        for j in 0..i {
            let (lower_j, upper_j) = fresh_ingredients_vec[j];

            // skip invalid ranges
            if lower_j > upper_j {
                continue;
            }

            if current <= upper_j && current >= lower_j {
                fresh_ingredients_vec[i] = (upper_j + 1, upper);
                current = upper_j + 1;
            }
        }

        // Upper
        (lower, upper) = fresh_ingredients_vec[i]; // get the latest values
        current = upper;

        for j in 0..i {
            let (lower_j, upper_j) = fresh_ingredients_vec[j];

            // skip invalid ranges
            if lower_j > upper_j {
                continue;
            }

            if current <= upper_j && current >= lower_j {
                fresh_ingredients_vec[i] = (lower, lower_j - 1);
                current = lower_j - 1;
            }
        }
    }

    fresh_ingredients_vec.iter().for_each(|(lower, upper)| {
        if lower <= upper {
            result_part2 += upper - lower + 1;
        }
    });

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}
