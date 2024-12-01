use std::fs;

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let space_regex = Regex::new(r" +").unwrap();

    let (left_numbers, right_numbers): (Vec<_>, Vec<_>) = input_lines
        .map(|line| {
            let binding = space_regex.replace(line, " ");
            binding
                .trim()
                .split_once(" ")
                .map(|f| (f.0.parse::<i32>().unwrap(), f.1.parse::<i32>().unwrap()))
                .unwrap()
        })
        .unzip();

    let part1_sum = part1((left_numbers.clone(), right_numbers.clone()));
    let part2_sum = part2((left_numbers, right_numbers));

    println!("Part 1: {} Part 2 {} Took {:?} ", part1_sum, part2_sum, 0)
}

fn part1((mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>)) -> i32 {
    left_numbers.sort();
    right_numbers.sort();

    let pairs = left_numbers
        .into_iter()
        .zip(right_numbers.into_iter())
        .collect::<Vec<_>>();

    println!("Numbers: {:?}", pairs);

    let sum = pairs.iter().map(|f| (f.0 - f.1).abs()).sum::<i32>();

    sum
}

fn part2((mut left_numbers, mut right_numbers): (Vec<i32>, Vec<i32>)) -> i32 {
    left_numbers.sort();
    right_numbers.sort();

    let right_numbers = right_numbers.iter().enumerate().collect::<Vec<_>>();

    let mut sum = 0;

    for i in left_numbers {
        let mut nums = 0;
        for j in right_numbers.iter() {
            if i == *j.1 {
                nums = nums + 1;
            }
            if i != *j.1 && nums != 0 {
                break;
            }
        }
        sum = sum + (nums * i);
    }

    sum
}
