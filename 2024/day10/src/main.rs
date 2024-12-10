use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

use colored::Colorize;

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    // let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

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

    let mut to_travel: VecDeque<(u32, (usize, usize))> = VecDeque::new();
    let mut travel_paths: HashMap<(usize, usize), Vec<(u32, (usize, usize))>> = HashMap::new();
    let mut trailheads: Vec<(u32, (usize, usize))> = Vec::new();
    print_board(&height_map, &travel_paths);
    println!();

    for m in 0..height_map.len() {
        for n in 0..height_map[m].len() {
            let num = height_map[m][n];
            if num == 0 {
                to_travel.push_back((num, (m as usize, n as usize)));
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

    while let Some((num, pos)) = to_travel.pop_front() {
        let m = pos.0;
        let n = pos.1;

        let up = get_num(m as isize - 1, n as isize);

        let down = get_num(m as isize + 1, n as isize);
        let left = get_num(m as isize, n as isize - 1);
        let right = get_num(m as isize, n as isize + 1);

        if let Some((dr_num, dr_pos)) = up {
            if dr_num == num + 1 {
                to_travel.push_back((dr_num, dr_pos));
                let mut current_path = travel_paths.remove(&pos).unwrap_or(Vec::new());
                current_path.push((dr_num, dr_pos));
                travel_paths.insert(pos, current_path);
            }
        }
        if let Some((dr_num, dr_pos)) = down {
            if dr_num == num + 1 {
                to_travel.push_back((dr_num, dr_pos));
                let mut current_path = travel_paths.remove(&pos).unwrap_or(Vec::new());
                current_path.push((dr_num, dr_pos));
                travel_paths.insert(pos, current_path);
            }
        }
        if let Some((dr_num, dr_pos)) = left {
            if dr_num == num + 1 {
                to_travel.push_back((dr_num, dr_pos));
                let mut current_path = travel_paths.remove(&pos).unwrap_or(Vec::new());
                current_path.push((dr_num, dr_pos));
                travel_paths.insert(pos, current_path);
            }
        }
        if let Some((dr_num, dr_pos)) = right {
            if dr_num == num + 1 {
                to_travel.push_back((dr_num, dr_pos));
                let mut current_path = travel_paths.remove(&pos).unwrap_or(Vec::new());
                current_path.push((dr_num, dr_pos));
                travel_paths.insert(pos, current_path);
            }
        }
    }

    print_board(&height_map, &travel_paths);

    println!();
    println!("Paths: {:?}", travel_paths);
    println!();

    let mut cached: HashMap<(u32, (usize, usize)), Vec<(u32, (usize, usize))>> = HashMap::new();

    let part_1 = trailheads
        .iter()
        .map(|f| {
            let paths = travel_next(&travel_paths, f, &mut cached);
            let single = paths.iter().collect::<HashSet<_>>();
            single.len()
        })
        .sum::<usize>();

    println!("Part 1: {}", part_1);
    println!("Time: {:?}", start.elapsed());
}

fn travel_next(
    travel_paths: &HashMap<(usize, usize), Vec<(u32, (usize, usize))>>,
    current: &(u32, (usize, usize)),
    cached: &mut HashMap<(u32, (usize, usize)), Vec<(u32, (usize, usize))>>,
) -> Vec<(u32, (usize, usize))> {
    if cached.contains_key(current) {
        return cached.get(current).unwrap().clone();
    }

    let erg: Vec<(u32, (usize, usize))> = {
        if current.0 == 9 {
            return vec![*current];
        }

        let next = travel_paths.get(&current.1);

        if next.is_none() {
            return vec![];
        }

        next.unwrap()
            .iter()
            .map(|next| travel_next(travel_paths, next, cached))
            .flatten()
            .collect()
    };

    cached.insert(*current, erg.clone());
    return erg;
}

fn print_board(
    height_map: &Vec<Vec<u32>>,
    travel_paths: &HashMap<(usize, usize), Vec<(u32, (usize, usize))>>,
) {
    for m in 0..height_map.len() {
        for n in 0..height_map[m].len() {
            let num = height_map[m][n];
            let path = travel_paths.get(&(m as usize, n as usize));

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
