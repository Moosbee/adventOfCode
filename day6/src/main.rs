use std::fs;

use regex::Regex;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let erg_part_1 = part1(&input);
    println!("Solution1: {} took {:?}", erg_part_1, start.elapsed());
    let erg_part_2 = part2(&input);
    println!("Solution2: {} took {:?}", erg_part_2, start.elapsed());
}

fn part1(input: &str) -> i128 {
    let spaces = Regex::new(r"(  +)").expect("Invalid regex");

    let small_line = spaces.replace_all(&input, " ");

    let input_lines: Vec<&str> = small_line.lines().collect();

    println!("Files Lines {}", input_lines.len());

    if input_lines.len() == 2 {
        let times: Vec<i128> = input_lines[0]
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let distances: Vec<i128> = input_lines[1]
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        println!("Times:{:?}\nDistance:{:?}", times, distances);

        let mut erg = 1;

        for index in 0..times.len().min(distances.len()) {
            let race = (times[index], distances[index]);

            let possible_races: Vec<(i128, i128)> = calc_distances(race.0);

            let winning_races: Vec<&(i128, i128)> = possible_races
                .iter()
                .filter(|r| {
                    // println!("{:?} {} {} {}", r, r.0, r.1,r.1 > race.1);
                    r.1 > race.1
                })
                .collect();

            println!(
                "Race time:{} Record:{} Possible races:{:?} Winning races:{:?}",
                race.0, race.1, possible_races, winning_races
            );
            erg = erg * winning_races.len()
        }

        let return_number: i128 = erg.try_into().unwrap();

        return return_number;
    }

    0
}

fn part2(input: &str) -> i128 {
    let small_line = input.replace(" ", "");

    let input_lines: Vec<&str> = small_line.lines().collect();

    println!("Files Lines {}", input_lines.len());

    if input_lines.len() == 2 {
        let time: i128 = input_lines[0]
            .trim()
            .strip_prefix("Time:")
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();
        let distance: i128 = input_lines[1]
            .trim()
            .strip_prefix("Distance:")
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();

        println!("Times:{}\nDistance:{}", time, distance);

        let possible_races: Vec<(i128, i128)> = calc_distances(time);

        let winning_races: Vec<&(i128, i128)> = possible_races
            .iter()
            .filter(|r| {
                // println!("{:?} {} {} {}", r, r.0, r.1,r.1 > distance);
                r.1 > distance
            })
            .collect();

        println!(
            "Race time:{} Record:{} Possible races:{:?} Winning races:{:?}, w start:{} w end:{}",
            time,
            distance,
            possible_races.len(),
            winning_races.len(),
            winning_races.first().unwrap().0,
            winning_races.last().unwrap().0
        );

        return winning_races.len().try_into().unwrap();
    }
    0
}

fn calc_distances(time: i128) -> Vec<(i128, i128)> {
    let mut times: Vec<(i128, i128)> = vec![];

    for use_time in 0..time + 1 {
        times.push((use_time, calc_distance(use_time, time)));
    }

    times
}

fn calc_distance(button_time: i128, max_time: i128) -> i128 {
    let speed = button_time;
    let drive_time = max_time - button_time;

    drive_time * speed
}
