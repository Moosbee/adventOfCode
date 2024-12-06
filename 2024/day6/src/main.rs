use colored::Colorize;
use std::{collections::HashSet, fs, time::Instant};

#[derive(Debug, Clone)]
struct Position {
    obstacle: bool,
    visited: bool,

    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let mut guard: Guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };

    let path: Vec<Vec<Position>> = input_lines
        .enumerate()
        .map(|line| {
            line.1
                .trim()
                .chars()
                .enumerate()
                .map(|l| {
                    if l.1 == '^' {
                        guard.x = l.0;
                        guard.y = line.0;
                    }

                    Position {
                        obstacle: l.1 == '#',
                        visited: l.1 == '^',
                        up: false,
                        down: false,
                        left: false,
                        right: false,
                    }
                })
                .collect()
        })
        .collect();

    let count_part_1 = calc_steps(path.clone(), guard.clone());

    print_area(&path);

    let mut count_part_2 = 0;

    for i in 0..path.len() {
        for j in 0..path[i].len() {
            let mut new_path = path.clone();
            new_path[i][j].obstacle = true;
            let count = calc_steps(new_path.clone(), guard.clone());

            println!("{} {} {}", i, j, count);

            if count < 10 {
                count_part_2 += 1;
            }
        }
    }

    println!(
        "Part 1: {} Part 2 {}(2057 is too high) Took {:?} ",
        count_part_1,
        count_part_2,
        start.elapsed()
    )
}

fn get_next_pos_mut(r: isize, col: isize, path: &mut Vec<Vec<Position>>) -> Option<&mut Position> {
    if r >= 0 && col >= 0 {
        path.get_mut(r as usize)
            .and_then(|colon| colon.get_mut(col as usize))
    } else {
        None
    }
}

fn get_next_pos(r: isize, col: isize, path: &Vec<Vec<Position>>) -> Option<&Position> {
    if r >= 0 && col >= 0 {
        path.get(r as usize)
            .and_then(|colon| colon.get(col as usize))
    } else {
        None
    }
}

fn calc_steps(mut path: Vec<Vec<Position>>, mut guard: Guard) -> i32 {
    let mut count = 1;
    for _ in 0..1_000_000 {
        // for _ in 0..path.len() * path[0].len() {
        let next = match guard.direction {
            Direction::Up => (guard.y as isize - 1, guard.x as isize),
            Direction::Down => (guard.y as isize + 1, guard.x as isize),
            Direction::Left => (guard.y as isize, guard.x as isize - 1),
            Direction::Right => (guard.y as isize, guard.x as isize + 1),
        };

        let next_pos = get_next_pos_mut(next.0, next.1, &mut path);

        if next_pos.is_none() {
            return count;
        }

        let next_pos = next_pos.unwrap();

        let has_visited = match guard.direction {
            Direction::Up => next_pos.right,
            Direction::Right => next_pos.down,
            Direction::Down => next_pos.left,
            Direction::Left => next_pos.up,
        };

        if has_visited {
            println!("Loop detected at {} {}", guard.x, guard.y);
            return -1;
        }

        if next_pos.obstacle {
            match guard.direction {
                Direction::Up => guard.direction = Direction::Right,
                Direction::Right => guard.direction = Direction::Down,
                Direction::Down => guard.direction = Direction::Left,
                Direction::Left => guard.direction = Direction::Up,
            };
        } else {
            match guard.direction {
                Direction::Up => next_pos.right = true,
                Direction::Right => next_pos.down = true,
                Direction::Down => next_pos.left = true,
                Direction::Left => next_pos.up = true,
            };

            if !next_pos.visited {
                count += 1;
            }

            next_pos.visited = true;

            guard.x = next.1 as usize;
            guard.y = next.0 as usize;
        }
    }

    -1
}

fn print_area(path: &Vec<Vec<Position>>) {
    for line in path {
        for pos in line {
            let text = if pos.obstacle {
                "#".to_string()
            } else {
                ".".to_string()
            };

            if pos.visited {
                print!("{}", text.yellow());
            } else {
                print!("{}", text);
            }
        }
        println!("");
    }
}
