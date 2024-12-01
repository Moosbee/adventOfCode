use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut sum = 0;

    for line in input_lines {
        let mut numbers_all: Vec<Vec<i32>> =
            vec![line.split(" ").filter_map(|f| f.parse().ok()).collect()];

        loop {
            let last_numbers = numbers_all.last().unwrap();
            if last_numbers.iter().all(|f| f == &0) {
                break;
            }

            numbers_all.push(calc_div(last_numbers));
        }

        numbers_all.reverse();

        println!("Numbers:{:?}", numbers_all);

        let mut old_num = 0;

        for numbers in numbers_all {
            old_num =  numbers.first().unwrap() - old_num;
        }

        println!("num: {}", old_num);
        sum = sum + old_num;
    }

    println!("Solution: {} took {:?}", sum, start.elapsed());
}

fn calc_div(numbers: &Vec<i32>) -> Vec<i32> {
    let mut div_vec: Vec<i32> = vec![];

    for index in 1..numbers.len() {
        div_vec.push(numbers[index] - numbers[index - 1]);
    }

    div_vec
}
