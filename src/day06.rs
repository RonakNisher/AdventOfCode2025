use itertools::Itertools;
use std::panic;

fn get_result(current_operator: &char, current_operands: &Vec<i64>) -> i64 {
    match current_operator {
        '+' => current_operands.iter().sum(),
        '*' => current_operands.iter().product(),
        _ => panic!("Invalid operator: {}", current_operator),
    }
}

pub fn solve(input: String) {
    let mut result: i64 = 0;
    let mut result_part2: i64 = 0;

    let lines = input.lines().collect_vec();
    let input_size = lines.len();
    let max_operands = lines[0].split_whitespace().count();
    let mut operands: Vec<Vec<i64>> = vec![Vec::new(); max_operands];
    for line in &lines[0..input_size - 1] {
        let ops = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();

        for i in 0..ops.len() {
            operands[i].push(ops[i]);
        }
    }

    let operators = lines[input_size - 1]
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect_vec();

    ///////////////
    // part 1
    ///////////////
    operators
        .iter()
        .zip(&operands)
        .for_each(|(operator, ops)| result += get_result(operator, ops));

    ///////////////
    // part 2
    ///////////////

    let max_len = lines
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    let part2_lines = input.lines().collect_vec();
    let mut current_operator: char = ' ';
    let mut current_operands: Vec<i64> = Vec::with_capacity(input_size - 1);
    for i in 0..max_len {
        let operator = part2_lines[input_size - 1].chars().nth(i).unwrap_or(' ');
        if operator != ' ' {
            current_operator = operator;
        }

        let column: String = part2_lines
            .iter()
            .take(input_size - 1)
            .map(|line| line.chars().nth(i).unwrap_or(' '))
            .filter(|&x| x != ' ')
            .collect();

        if column.is_empty() {
            result_part2 += get_result(&current_operator, &current_operands);

            current_operands.clear();
            current_operator = ' ';
            continue;
        }

        current_operands.push(column.parse::<i64>().unwrap());
    }

    // last one
    if !current_operands.is_empty() {
        result_part2 += get_result(&current_operator, &current_operands);
    }

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
