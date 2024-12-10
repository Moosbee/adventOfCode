use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let height_map = input_lines
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap_or(100))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut to_travel: VecDeque<((u32, (usize, usize)), (u32, (usize, usize)))> = VecDeque::new();
    let mut travel_paths: HashMap<(u32, (usize, usize)), HashSet<(u32, (usize, usize))>> =
        HashMap::new();
    let mut trailheads: Vec<(u32, (usize, usize))> = Vec::new();
    print_board(&height_map, &travel_paths);
    println!();

    for m in 0..height_map.len() {
        for n in 0..height_map[m].len() {
            let num = height_map[m][n];
            if num == 0 {
                to_travel.push_back((
                    (num, (m as usize, n as usize)),
                    (num, (m as usize, n as usize)),
                ));
            }
            if num == 9 {
                trailheads.push((num, (m as usize, n as usize)));
            }
        }
    }

    let get_num = |r: isize, col: isize| -> Option<(u32, (usize, usize))> {
        if r >= 0 && col >= 0 {
            height_map
                .get(r as usize)
                .map(|colon| colon.get(col as usize))
                .flatten()
                .cloned()
                .zip(Some((r as usize, col as usize)))
        } else {
            None
        }
    };

    while let Some(((num, pos), start)) = to_travel.pop_front() {
        let m = pos.0;
        let n = pos.1;

        let current = (num, pos);

        let mut path = travel_paths.remove(&current).unwrap_or_default();
        path.insert(start);
        travel_paths.insert(current, path);

        let up = get_num(m as isize - 1, n as isize);

        let down = get_num(m as isize + 1, n as isize);
        let left = get_num(m as isize, n as isize - 1);
        let right = get_num(m as isize, n as isize + 1);

        if let Some((dr_num, dr_pos)) = up {
            if dr_num == num + 1 {
                to_travel.push_back(((dr_num, dr_pos), start));
            }
        }
        if let Some((dr_num, dr_pos)) = down {
            if dr_num == num + 1 {
                to_travel.push_back(((dr_num, dr_pos), start));
            }
        }
        if let Some((dr_num, dr_pos)) = left {
            if dr_num == num + 1 {
                to_travel.push_back(((dr_num, dr_pos), start));
            }
        }
        if let Some((dr_num, dr_pos)) = right {
            if dr_num == num + 1 {
                to_travel.push_back(((dr_num, dr_pos), start));
            }
        }
    }

    print_board(&height_map, &travel_paths);

    println!();
    // println!("Trailheads: {:?}", travel_paths);
    let res = travel_paths
        .iter()
        .filter(|pt| pt.0 .0 == 9)
        .map(|pt| pt.1.iter().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    println!();

    println!("Part 1: {}", res.len());
    println!("Time: {:?}", start.elapsed());
}

fn print_board(
    height_map: &Vec<Vec<u32>>,
    travel_paths: &HashMap<(u32, (usize, usize)), HashSet<(u32, (usize, usize))>>,
) {
    for m in 0..height_map.len() {
        for n in 0..height_map[m].len() {
            let num = height_map[m][n];
            let path = travel_paths.get(&(num, (m as usize, n as usize)));

            if let Some(path) = path {
                match num {
                    0 => print!("{}", height_map[m][n].to_string().green()),
                    1 => print!("{}", height_map[m][n].to_string().blue()),
                    2 => print!("{}", height_map[m][n].to_string().yellow()),
                    3 => print!("{}", height_map[m][n].to_string().magenta()),
                    4 => print!("{}", height_map[m][n].to_string().purple()),
                    5 => print!("{}", height_map[m][n].to_string().cyan()),
                    6 => print!("{}", height_map[m][n].to_string().bright_green()),
                    7 => print!("{}", height_map[m][n].to_string().bright_magenta()),
                    8 => print!("{}", height_map[m][n].to_string().bright_blue()),
                    9 => print!("{}", height_map[m][n].to_string().red()),
                    _ => print!("{}", height_map[m][n].to_string().white()),
                }
            } else {
                print!("{}", height_map[m][n]);
            }
        }
        println!("");
    }
}

/// Convert a HashMap of (row, column) to value into a 2D vector
///
/// The size of the output vector is determined by the maximum row and column
/// present in the input HashMap.
///
/// # Example
///
///
fn hashmap_to_vec<T: Default + Clone>(map: HashMap<(usize, usize), T>) -> Vec<Vec<T>> {
    // Determine the size of the grid
    let (mut max_row, mut max_col) = (0, 0);
    for &(row, col) in map.keys() {
        if row > max_row {
            max_row = row;
        }
        if col > max_col {
            max_col = col;
        }
    }

    // Initialize a 2D vector with default values
    let mut grid = vec![vec![T::default(); max_col + 1]; max_row + 1];

    // Populate the grid with values from the HashMap
    for ((row, col), value) in map {
        grid[row][col] = value;
    }

    grid
}
