use std::{fs, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let calculations = input_lines
        .map(|line| {
            let spl = line.split_once(':').unwrap();
            let sum = spl.0.trim().parse::<u64>().unwrap();
            let parts = spl
                .1
                .trim()
                .split(' ')
                .map(|f| f.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            (sum, parts)
        })
        .collect::<Vec<_>>();

    let sum_part_1: u64 = calculations
        .par_iter()
        .filter(|f| can_calculate(f.0, f.1.clone(), true))
        .map(|f| f.0)
        .sum::<u64>();

    let sum_part_2: u64 = calculations
        .par_iter()
        .filter(|f| can_calculate(f.0, f.1.clone(), false))
        .map(|f| f.0)
        .sum::<u64>();

    println!("Part 1 {}", sum_part_1);
    println!("Part 2 {}", sum_part_2);
    println!("Took {:?} ", start.elapsed());
}

fn can_calculate(sum: u64, parts: Vec<u64>, simple: bool) -> bool {
    let mut parts = parts
        .iter()
        .map(|f| (Operation::Add, *f))
        .collect::<Vec<_>>();

    loop {
        let erg = calc_vec(&parts);

        if erg == sum {
            return true;
        }

        let next_ones = if simple {
            next_vec_simple(parts)
        } else {
            next_vec(parts)
        };

        if next_ones.1 {
            return false;
        };

        parts = next_ones.0;
    }
}

enum Operation {
    Add,
    Multiply,
    Concat,
}

fn next_vec_simple(mut parts: Vec<(Operation, u64)>) -> (Vec<(Operation, u64)>, bool) {
    let mut carry = true;

    for part in parts.iter_mut().skip(1) {
        if carry {
            match part.0 {
                Operation::Add => {
                    carry = false;
                    part.0 = Operation::Multiply;
                }
                Operation::Multiply => {
                    carry = true;
                    part.0 = Operation::Add;
                }
                _ => panic!(),
            }
        }
    }

    (parts, carry)
}

fn next_vec(mut parts: Vec<(Operation, u64)>) -> (Vec<(Operation, u64)>, bool) {
    let mut carry = true;

    for part in parts.iter_mut().skip(1) {
        if carry {
            match part.0 {
                Operation::Add => {
                    carry = false;
                    part.0 = Operation::Multiply;
                }
                Operation::Multiply => {
                    carry = false;
                    part.0 = Operation::Concat;
                }
                Operation::Concat => {
                    carry = true;
                    part.0 = Operation::Add;
                }
            }
        }
    }

    (parts, carry)
}

fn calc_vec(parts: &Vec<(Operation, u64)>) -> u64 {
    parts.iter().fold(0, |acc, f| match f.0 {
        Operation::Add => acc + f.1,
        Operation::Multiply => acc * f.1,
        Operation::Concat => concat_numbers(acc, f.1),
    })
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut multiplier = 1;
    let mut temp = b;

    // Find the multiplier to shift `a` to the left
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    // Combine the numbers
    a * multiplier + b
}
