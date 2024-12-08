use std::{fs, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    // let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let calculations = input_lines
        .map(|line| {
            let spl = line.split_once(':').unwrap();
            let sum = spl.0.trim().parse::<i64>().unwrap();
            let parts = spl
                .1
                .trim()
                .split(' ')
                .map(|f| f.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (sum, parts)
        })
        .collect::<Vec<_>>();

    let sum: i64 = calculations
        .iter()
        .filter(|f| can_calculate(f.0, f.1.clone()))
        .map(|f| f.0)
        .sum::<i64>();

    println!("Solution 1: {} Took {:?}", sum, start.elapsed());
}

fn can_calculate(sum: i64, parts: Vec<i64>) -> bool {
    let mut parts = parts
        .iter()
        .map(|f| (Operation::Add, *f))
        .collect::<Vec<_>>();

    loop {
        let erg = calc_vec(&parts);

        if erg == sum {
            return true;
        }

        let next_ones = next_vec(parts);

        if next_ones.1 {
            return false;
        };

        parts = next_ones.0;
    }
}

enum Operation {
    Add,
    Multiply,
}

fn next_vec(mut parts: Vec<(Operation, i64)>) -> (Vec<(Operation, i64)>, bool) {
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
            }
        }
    }

    (parts, carry)
}

fn calc_vec(parts: &Vec<(Operation, i64)>) -> i64 {
    parts.iter().fold(0, |acc, f| match f.0 {
        Operation::Add => acc + f.1,
        Operation::Multiply => acc * f.1,
    })
}
