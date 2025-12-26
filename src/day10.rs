use itertools::Itertools;
use std::collections::VecDeque;

fn does_pattern_match(target: &Vec<char>, current: &Vec<char>) -> bool {
    for i in 0..target.len() {
        if target[i] != current[i] {
            return false;
        }
    }
    return true;
}

struct PatternState {
    pattern: Vec<char>,
    presses: i32,
}

fn get_smallest_switch_presses_bfs(target_pattern: &Vec<char>, switches: &Vec<Vec<i32>>) -> u128 {
    let result;

    let mut patterns_queue: VecDeque<PatternState> = VecDeque::new();
    let mut current_pattern: Vec<char> = vec!['.'; target_pattern.len()];
    patterns_queue.push_back(PatternState {
        pattern: current_pattern.clone(),
        presses: 0,
    });
    loop {
        let current_state = patterns_queue.pop_front().unwrap();

        if does_pattern_match(target_pattern, &current_state.pattern) {
            // found a match
            result = current_state.presses as u128;
            break;
        }

        if current_state.presses > 10 {
            continue;
        }

        current_pattern = current_state.pattern;

        for switch in switches {
            // toggle the switch
            let mut new_pattern = current_pattern.clone();
            for &index in switch {
                let idx = index as usize;
                if new_pattern[idx] == '#' {
                    new_pattern[idx] = '.';
                } else {
                    new_pattern[idx] = '#';
                }
            }

            if patterns_queue
                .iter()
                .any(|state| state.pattern == new_pattern)
            {
                // already visited this pattern
                continue;
            }

            patterns_queue.push_back(PatternState {
                pattern: new_pattern,
                presses: current_state.presses + 1,
            });
        }
    }

    return result;
}

pub fn solve(input: String) {
    let mut result: u128 = 0;

    input.lines().for_each(|line| {
        let re = regex::Regex::new(r"\[([.#]+)\] ([\(\)0-9, ]+) \{[0-9,]+\}").unwrap();
        let matches = re.captures(line).unwrap();

        let patterns = &matches[1].chars().collect::<Vec<char>>();
        let mut switches: Vec<Vec<i32>> = vec![];
        matches[2].split(" ").collect_vec().iter().for_each(|x| {
            let switch = x
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .collect_vec()
                .iter()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            switches.push(switch);
        });

        result += get_smallest_switch_presses_bfs(&patterns, &switches);
    });

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("*******************");
}
