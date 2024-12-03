use std::{fs, time::Instant};

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./test_input_2.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let part1_sum = part1(input.clone());

    let part2_sum = part2(input.clone());

    println!(
        "Part 1: {} Part 2 {} Took {:?}",
        part1_sum,
        part2_sum,
        start.elapsed()
    );
}

fn part1(input: String) -> i32 {
    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let instructions = mul_regex
        .find_iter(&input)
        .map(|f| f.as_str())
        .collect::<Vec<_>>();

    println!("Erg: {:?}", instructions);

    let mut sum = 0;

    for instruction in instructions {
        if mul_regex.is_match(instruction) {
            let (first, second) = instruction.split_once(',').unwrap();
            let num1 = first.replace("mul(", "").parse::<i32>().unwrap();
            let num2 = second.replace(")", "").parse::<i32>().unwrap();
            let erg = num1 * num2;
            sum = sum + erg;
            println!("{} * {} = {} {}", num1, num2, erg, sum);
        }
    }

    sum
}

fn part2(input: String) -> i32 {
    let mul_do_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let instructions = mul_do_regex
        .find_iter(&input)
        .map(|f| f.as_str())
        .collect::<Vec<_>>();

    println!("Erg: {:?}", instructions);

    let mut sum = 0;
    let mut active = true;

    for instruction in instructions {
        if mul_regex.is_match(instruction) {
            if active {
                let (first, second) = instruction.split_once(',').unwrap();
                let num1 = first.replace("mul(", "").parse::<i32>().unwrap();
                let num2 = second.replace(")", "").parse::<i32>().unwrap();
                let erg = num1 * num2;
                sum = sum + erg;
                println!("{} * {} = {} {}", num1, num2, erg, sum);
            }
        } else if instruction == "do()" {
            active = true;
        } else if instruction == "don't()" {
            active = false;
        }
    }

    sum
}
