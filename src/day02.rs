use itertools::Itertools;

pub fn solve(input: String) {
    let mut result: i64 = 0;
    let mut result_part2: i64 = 0;

    let limits: Vec<&str> = input.split(",").collect_vec();
    limits.iter().for_each(|line| {
        let (x, y) = line.split("-").collect_tuple().unwrap();
        let upper_limit = y.parse::<i64>().unwrap();

        let x_int = x.parse::<i64>().unwrap();
        let mut current = x_int;
        let mut current_str;

        while current <= upper_limit {
            current_str = current.to_string();

            let (first, last) = current_str.split_at(current_str.len() / 2);
            if first == last {
                result += current;
            }

            for i in 1..current_str.len() {
                let (first, mut last) = current_str.split_at(i);
                while last.starts_with(first) {
                    last = &last[first.len()..];
                }
                if last.is_empty() {
                    result_part2 += current;
                    break;
                }
            }

            current += 1;
        }
    });

    println!("*******************");
    println!("Solved Part 1: {}", result);
    println!("Solved Part 2: {}", result_part2);
    println!("*******************");
}
