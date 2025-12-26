use itertools::Itertools;

fn does_pattern_match(target: &Vec<char>, current: &Vec<char>) -> bool {
    for i in 0..target.len() {
        if target[i] != current[i] {
            return false;
        }
    }
    return true;
}

fn get_smallest_switch_presses(
    pattern: &Vec<char>,
    switches: &Vec<Vec<i32>>,
    current_presses: i32,
    current_pattern: &mut Vec<char>,
    smallest_presses: &mut i32,
) -> u128 {
    // early exit
    if current_presses >= *smallest_presses {
        return u128::MAX;
    }

    // base case
    if does_pattern_match(pattern, current_pattern) {
        if current_presses < *smallest_presses {
            *smallest_presses = current_presses;
            return current_presses as u128;
        }
    }

    let mut result: u128 = u128::MAX;

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

        let res = get_smallest_switch_presses(
            &pattern,
            switches,
            current_presses + 1,
            &mut new_pattern,
            smallest_presses,
        );

        if res < result {
            result = res;
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

        let mut smallest_presses = 10; //i32::MAX;
        let mut current_pattern: Vec<char> = vec!['.'; patterns.len()];
        let res = get_smallest_switch_presses(
            &patterns,
            &switches,
            0, /*current_presses*/
            &mut current_pattern,
            &mut smallest_presses,
        );
        result += res;
    });

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("*******************");
}
