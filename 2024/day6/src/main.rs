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

    let mut path: Vec<Vec<Position>> = input_lines
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

    let init_guard = guard.clone();

    let mut loops: HashSet<(isize, isize)> = HashSet::new();

    loop {
        let next = match guard.direction {
            Direction::Up => (guard.y as isize - 1, guard.x as isize),
            Direction::Down => (guard.y as isize + 1, guard.x as isize),
            Direction::Left => (guard.y as isize, guard.x as isize - 1),
            Direction::Right => (guard.y as isize, guard.x as isize + 1),
        };

        let is_loop = is_infinite_loop(path.clone(), guard.clone(), (next.0, next.1));

        if is_loop {
            loops.insert(next);
            println!(
                "Infinite loop detected at {} {} {}",
                next.0,
                next.1,
                loops.len()
            );
        }

        let next_pos = get_next_pos_mut(next.0, next.1, &mut path);

        if next_pos.is_none() {
            break;
        }

        let next_pos = next_pos.unwrap();

        let has_visited = match guard.direction {
            Direction::Up => next_pos.right,
            Direction::Right => next_pos.down,
            Direction::Down => next_pos.left,
            Direction::Left => next_pos.up,
        };

        if has_visited {
            println!("G Loop detected at {} {}", guard.x, guard.y);
        }

        if next_pos.obstacle {
            match guard.direction {
                Direction::Up => guard.direction = Direction::Right,
                Direction::Right => guard.direction = Direction::Down,
                Direction::Down => guard.direction = Direction::Left,
                Direction::Left => guard.direction = Direction::Up,
            };
        } else {
            next_pos.visited = true;

            match guard.direction {
                Direction::Up => next_pos.right = true,
                Direction::Right => next_pos.down = true,
                Direction::Down => next_pos.left = true,
                Direction::Left => next_pos.up = true,
            };

            // if !(init_guard.x as isize == next.1 && init_guard.y as isize == next.0) {

            // }

            guard.x = next.1 as usize;
            guard.y = next.0 as usize;
        }
    }

    print_area(&path);

    let mut count = 0;
    for line in path.iter() {
        for pos in line {
            if pos.visited {
                count += 1;
            }
        }
    }

    let mut possible_spots = path
        .iter()
        .enumerate()
        .map(|x| {
            x.1.iter()
                .enumerate()
                .filter(|x| !x.1.obstacle)
                .map(|y| (x.0 as isize, y.0 as isize))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();

    possible_spots.remove(&(init_guard.y as isize, init_guard.x as isize));

    let loops = loops.intersection(&possible_spots).collect::<Vec<_>>();

    println!("Loops: {:?} ", loops);

    println!(
        "Part 1: {} Part 2 {}(2057 is too high) Took {:?} ",
        count,
        loops.len(),
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

fn is_infinite_loop(
    mut path: Vec<Vec<Position>>,
    mut guard: Guard,
    extra_obstacle: (isize, isize),
) -> bool {
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
            return false;
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
            return true;
        }

        if next_pos.obstacle || (next.0 == extra_obstacle.0 && next.1 == extra_obstacle.1) {
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

            guard.x = next.1 as usize;
            guard.y = next.0 as usize;
        }
    }

    true
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
