use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    // let input =
    // fs::read_to_string("./test_input_2.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let max_x = input_lines.clone().count() as i32 - 1;
    let max_y = input_lines.clone().next().unwrap().trim().len() as i32 - 1;

    let mut frequency_antennas: HashMap<String, Vec<(i32, i32)>> = HashMap::new();

    for x in input_lines.enumerate().clone() {
        let line = x.1.trim().to_string();
        for y in line.chars().enumerate().clone() {
            if y.1 != '.' {
                frequency_antennas
                    .entry(y.1.to_string())
                    .or_insert(Vec::new())
                    .push((x.0 as i32, y.0 as i32));
            }
        }
    }

    println!("Frequency Antennas: {:#?}", frequency_antennas);

    let mut all_possible_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_key, value) in frequency_antennas.iter() {
        let antinodes = gen_possible_antinodes(value.clone(), 1, 1);

        all_possible_antinodes.extend(antinodes);
    }

    let all_antinodes = all_possible_antinodes
        .into_iter()
        .filter(|f| f.0 >= 0 && f.0 <= max_x && f.1 >= 0 && f.1 <= max_y)
        .collect::<Vec<_>>();

    // let fwr = frequency_antennas
    //     .iter()
    //     .map(|f| {
    //         f.1.clone()
    //             .into_iter()
    //             .map(|l| (f.0.clone(), l))
    //             .collect::<Vec<(String, (i32, i32))>>()
    //     })
    //     .flatten()
    //     .collect();

    // print_board(&fwr, &all_antinodes, max_x, max_y);

    println!("Part 1: {}", all_antinodes.len());

    let mut all_possible_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_key, value) in frequency_antennas.iter() {
        let antinodes = gen_possible_antinodes(value.clone(), max_x * max_y, 0);

        all_possible_antinodes.extend(antinodes);
    }

    let all_antinodes = all_possible_antinodes
        .into_iter()
        .filter(|f| f.0 >= 0 && f.0 <= max_x && f.1 >= 0 && f.1 <= max_y)
        .collect::<Vec<_>>();

    // let fwr = frequency_antennas
    //     .iter()
    //     .map(|f| {
    //         f.1.clone()
    //             .into_iter()
    //             .map(|l| (f.0.clone(), l))
    //             .collect::<Vec<(String, (i32, i32))>>()
    //     })
    //     .flatten()
    //     .collect();

    // print_board(&fwr, &all_antinodes, max_x, max_y);

    println!("Part 2: {}", all_antinodes.len());

    println!("Took {:?}", start.elapsed());
}

fn gen_possible_antinodes(nodes: Vec<(i32, i32)>, multiples: i32, min: i32) -> Vec<(i32, i32)> {
    let mut cross_nodes: Vec<((i32, i32), (i32, i32))> = nodes
        .iter()
        .map(|n| nodes.iter().map(|n2| (*n, *n2)))
        .flatten()
        .filter(|f| f.0 != f.1)
        .collect();

    dedup_with_eq(&mut cross_nodes, |a, b| {
        (a.0 == b.0 && a.1 == b.1) || (a.1 == b.0 && a.0 == b.1)
    });

    // println!("Cross Nodes: {:#?}", cross_nodes);

    let antinodes = cross_nodes
        .iter()
        .map(|n| {
            let diff = ((n.0 .0 - n.1 .0), (n.0 .1 - n.1 .1));

            let mut antinodes = vec![];

            for i in min..=multiples {
                let antinode_1 = (n.0 .0 + diff.0 * i, n.0 .1 + diff.1 * i);
                let antinode_2 = (n.1 .0 - diff.0 * i, n.1 .1 - diff.1 * i);
                antinodes.push(antinode_1);
                antinodes.push(antinode_2);
            }
            antinodes
        })
        .flatten()
        .collect::<Vec<_>>();

    // println!("Antinodes: {:#?}", antinodes);

    antinodes
}

fn print_board(
    nodes: &Vec<(String, (i32, i32))>,
    antinodes: &Vec<(i32, i32)>,
    max_x: i32,
    max_y: i32,
) {
    for x in 0..max_x {
        for y in 0..max_y {
            if let Some(node) = nodes.iter().find(|f| f.1 .0 == x && f.1 .1 == y) {
                print!("{}", node.0);
            } else if antinodes.iter().find(|f| f.0 == x && f.1 == y).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dedup_with_eq<T, F>(vec: &mut Vec<T>, eq_fn: F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut i = 0;
    while i < vec.len() {
        let mut j = i + 1;
        while j < vec.len() {
            if eq_fn(&vec[i], &vec[j]) {
                vec.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}
