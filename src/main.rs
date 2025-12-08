use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let day = &args[1];
    println!("Day: {}", day);

    let filepath = format!("src/inputs/day{}_input.txt", day);
    println!("Filepath: {}", filepath);
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    match day.as_str() {
        "01" => day01::solve(contents),
        "02" => day02::solve(contents),
        "03" => day03::solve(contents),
        "04" => day04::solve(contents),
        "05" => day05::solve(contents),
        "06" => day06::solve(contents),
        "07" => day07::solve(contents),
        "08" => day08::solve(contents),
        _ => println!("No solution for day {}", day),
    }
}
