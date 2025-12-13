use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State {
    current_cable: String,
    seen_dac: bool,
    seen_fft: bool,
}

fn get_unreachable_nodes(
    from: &str,
    cables_map: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    return cables_map
        .keys()
        .filter(|&cable| {
            let mut visited: HashSet<String> = HashSet::new();
            let mut stack: Vec<String> = vec![cable.to_string()];

            let mut can_reach = false;

            while let Some(current) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current.clone());

                if current == from {
                    can_reach = true;
                }

                if let Some(neighbors) = cables_map.get(&current) {
                    for neighbor in neighbors {
                        stack.push(neighbor.clone());
                    }
                }
            }

            !(can_reach)
        })
        .cloned()
        .collect();
}

fn traverse_new(
    current_cable: &str,
    cables_map: &HashMap<String, HashSet<String>>,
    current_path: &mut Vec<String>,
    visited_timelines_map: &mut HashMap<State, u128>,
    nodes_cannot_reach_dac: &HashSet<String>,
    nodes_cannot_reach_fft: &HashSet<String>,
    is_part_2: bool,
) -> u128 {
    current_path.push(current_cable.to_string());

    if current_cable == "out" {
        if !is_part_2 {
            return 1;
        } else {
            if current_path.contains(&"dac".to_string())
                && current_path.contains(&"fft".to_string())
            {
                return 1;
            } else {
                // invalid path
                return 0;
            }
        }
    }

    let state = State {
        current_cable: current_cable.to_string(),
        seen_dac: current_path.contains(&"dac".to_string()),
        seen_fft: current_path.contains(&"fft".to_string()),
    };

    if is_part_2 {
        if !state.seen_dac && nodes_cannot_reach_dac.contains(current_cable) {
            return 0;
        }

        if !state.seen_fft && nodes_cannot_reach_fft.contains(current_cable) {
            return 0;
        }
    }

    if visited_timelines_map.contains_key(&state) {
        // already visited
        return *visited_timelines_map.get(&state).unwrap();
    }

    let mut timelines = 0;
    for next_cable in cables_map.get(current_cable).unwrap() {
        let mut new_path = current_path.clone();
        let result = traverse_new(
            &next_cable,
            &cables_map,
            &mut new_path,
            visited_timelines_map,
            nodes_cannot_reach_dac,
            nodes_cannot_reach_fft,
            is_part_2,
        );

        new_path.push(next_cable.to_string());

        let new_state = State {
            current_cable: next_cable.to_string(),
            seen_dac: state.seen_dac || next_cable == "dac",
            seen_fft: state.seen_fft || next_cable == "fft",
        };

        visited_timelines_map.entry(new_state).or_insert(result);
        timelines += result;
    }

    return timelines;
}

pub fn solve(input: String) {
    let result: u128;
    let result_part2: u128;

    let mut cables_map: HashMap<String, HashSet<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let (cable, rest) = line.split_once(": ").unwrap();
        let connections: HashSet<String> = rest
            .split(" ")
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        cables_map.insert(cable.to_string(), connections);
    });

    // pre calculate which nodes have no path to dac and fft
    let nodes_cannot_reach_dac = get_unreachable_nodes("dac", &cables_map);
    let nodes_cannot_reach_fft = get_unreachable_nodes("fft", &cables_map);

    let mut current_path: Vec<String> = vec![];
    let mut visited_timelines_map_new: HashMap<State, u128> = HashMap::new();
    result = traverse_new(
        "you",
        &cables_map,
        &mut current_path,
        &mut visited_timelines_map_new,
        &nodes_cannot_reach_dac,
        &nodes_cannot_reach_fft,
        false, /*is_part_2*/
    );

    visited_timelines_map_new.clear();
    current_path.clear();
    result_part2 = traverse_new(
        "svr",
        &cables_map,
        &mut current_path,
        &mut visited_timelines_map_new,
        &nodes_cannot_reach_dac,
        &nodes_cannot_reach_fft,
        true, /*is_part_2*/
    );

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
