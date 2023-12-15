use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_parts = input.split(',');

    println!("Files Lines {}", input_parts.clone().count());

    println!("Erg Part 1: {} Took {:?}", 0, start.elapsed())
}
