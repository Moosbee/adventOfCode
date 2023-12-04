use std::fs;

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());
    let separator = Regex::new(r"([:|]+)").expect("Invalid regex");
    let spaces = Regex::new(r"(  +)").expect("Invalid regex");

    let mut total = 0;

    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    for line in input_lines {
        let small_line = spaces.replace_all(line, " ");
        let parts: Vec<&str> = separator.split(&small_line).into_iter().collect();

        if parts.len() == 3 {
            let id: i32 = parts[0]
                .trim()
                .strip_prefix("Card ")
                .and_then(|s| s.parse().ok())
                .unwrap_or_default();

            let winning_numbers: Vec<i32> = parts[1]
                .trim()
                .split(" ")
                .map(|s| s.parse().ok().unwrap_or_default())
                .collect();
            let my_numbers: Vec<i32> = parts[2]
                .trim()
                .split(" ")
                .map(|s| s.parse().ok().unwrap_or_default())
                .collect();

            let mut points = 0;

            for my_number in my_numbers.clone() {
                for win_number in winning_numbers.clone() {
                    if my_number == win_number {
                        points = (points * 2).max(1);
                    }
                }
            }

            total = total + points;

            println!(
                "Parts:{},id:{},winning:{:?},my:{:?},points:{},total:{}",
                parts.len(),
                id,
                winning_numbers,
                my_numbers,
                points,
                total
            );
        }
    }

    println!("total: {}", total);
}
