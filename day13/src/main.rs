use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let input_lines: Vec<&str> = input.split("\n\n").collect();

    println!("Files Lines {}", input_lines.len());

    let mut sum_part_1 = 0;

    for mirror in input_lines {
      sum_part_1 = sum_part_1 + part1(mirror);
    }

    println!("Part 1: {}", sum_part_1);

    println!("Took {:?}", start.elapsed());
}

fn part1(mirror: &str) -> usize {
    let lines: Vec<&str> = mirror.lines().collect();
    let mut line_indices: Vec<usize> = (0..lines[0].len()).collect();
    for line in &lines {
        line_indices = mirror_line_indexes(line_indices, &line.chars().collect());
        // println!("Line {} mirrors at {:?}", line, indices);
    }
    let mut column_indices: Vec<usize> = (0..lines.len()).collect();
    for column in 0..lines[0].len() {
        column_indices = mirror_column_indexes(column_indices, column, &lines)
        // println!("Line {} mirrors at {:?}", line, indices);
    }
    // println!(
    //     "Line\n{}\n mirrors at {:?} or {:?}",
    //     mirror,
    //     line_indices.first(),
    //     column_indices.first()
    // );
    println!(
        "Line mirrors at {:?} or {:?}",
        line_indices.first(),
        column_indices.first()
    );

    if line_indices.first().is_some() && column_indices.first().is_some() {
        println!("ops");
    } else if line_indices.first().is_some() && !column_indices.first().is_some() {
        return *line_indices.first().unwrap();
    } else if !line_indices.first().is_some() && column_indices.first().is_some() {
        return column_indices.first().unwrap() * 100;
    } else {
        println!("ops");
    }
    0
}

fn mirror_column_indexes(indices: Vec<usize>, column: usize, lines: &Vec<&str>) -> Vec<usize> {
    let line: Vec<char> = lines.iter().filter_map(|f| f.chars().nth(column)).collect();

    mirror_line_indexes(indices, &line)
}

fn mirror_line_indexes(indices: Vec<usize>, line: &Vec<char>) -> Vec<usize> {
    let mut new_indices = vec![];

    for index in indices {
        if mirror_on_line_char(index, line) {
            new_indices.push(index);
        }
    }
    new_indices
}

fn mirror_on_line_char(line_index: usize, line: &Vec<char>) -> bool {
    if line_index >= line.len() || line_index == 0 {
        return false;
    }
    let left_side_rev: Vec<char> = (&line[0..line_index])
        .iter()
        .rev()
        .map(|f| f.clone())
        .collect();
    let right_side: Vec<char> = (&line[line_index..]).iter().map(|f| f.clone()).collect();

    // println!("         Lin\n          {:?}\n          {:?}",left_side_rev,right_side);

    for index in [0..(left_side_rev.len().min(right_side.len()))] {
        if left_side_rev[index.clone()] != right_side[index] {
            return false;
        }
    }

    true
}
