use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

use colored::{Colorize, CustomColor};

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
    let mut travel_paths: HashMap<(u32, (usize, usize)), Vec<(u32, (usize, usize))>> =
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
        path.push(start);
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
    let part_1 = travel_paths
        .iter()
        .filter(|pt| pt.0 .0 == 9)
        .map(|pt| pt.1.iter().collect::<HashSet<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let part_2 = travel_paths
        .iter()
        .filter(|pt| pt.0 .0 == 9)
        .map(|pt| pt.1.iter().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    println!();

    println!("Part 1: {}", part_1.len());
    println!("Part 2: {}", part_2.len());
    println!("Time: {:?}", start.elapsed());
}

fn print_board(
    height_map: &Vec<Vec<u32>>,
    travel_paths: &HashMap<(u32, (usize, usize)), Vec<(u32, (usize, usize))>>,
) {
    for m in 0..height_map.len() {
        for n in 0..height_map[m].len() {
            let num = height_map[m][n];
            let path = travel_paths.get(&(num, (m as usize, n as usize)));

            if let Some(path) = path {
                let mut hasher = DefaultHasher::new();
                path.hash(&mut hasher);
                let num_color = hasher.finish() % (256 * 256 * 256);
                let color: CustomColor = CustomColor {
                    r: (num_color % 256) as u8,
                    g: ((num_color / 256) % 256) as u8,
                    b: ((num_color / (256 * 256)) % 256) as u8,
                };
                print!("{}", height_map[m][n].to_string().custom_color(color));
            } else {
                print!("{}", height_map[m][n]);
            }
        }
        println!("");
    }
}
