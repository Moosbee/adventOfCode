use std::fs;

use regex::Regex;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let spaces = Regex::new(r"(  +)").expect("Invalid regex");

    let small_line = spaces.replace_all(&input, " ");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    println!("inp: {}", small_line);
}
