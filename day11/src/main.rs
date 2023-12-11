use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    println!("Part 1: {} Part 2: {} took {:?}", 0, 0, start.elapsed());
}
