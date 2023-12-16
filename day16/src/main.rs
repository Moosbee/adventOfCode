use std::fs;

use std::time::Instant;

// use colored::Colorize;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let pipes: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    println!("Took {:?}", start.elapsed())
}
