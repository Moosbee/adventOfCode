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
    let mut sum_part_2 = 0;

    for mirror in input_lines {
        if false {
            sum_part_1 = sum_part_1 + part1(mirror);
        }
        sum_part_2 = sum_part_2 + part2(mirror);
    }

    println!("Part 1: {}", sum_part_1);
    println!("Part 2: {}", sum_part_2);

    println!("Took {:?}", start.elapsed());
}

fn part2(mirror: &str) -> usize {
    let lines: Vec<&str> = mirror.lines().collect();

    let line_smg = get_line_smudge(&lines);
    let column_smg = get_column_smudge(&lines);
    let line_mir = part1(mirror);

    if line_smg.is_some() {
        return (line_smg.unwrap().1-1)*100;
    }
    if column_smg.is_some() {
        return (column_smg.unwrap().1-1);
    }

    line_mir
}

fn fix_smudge(mirror: &str, smudge: (usize, usize)) -> String {
    let lines: Vec<&str> = mirror.lines().collect();

    let mut new_str: String = String::new();

    for line in lines.iter().enumerate() {
        if line.0 != smudge.1 {
            new_str = new_str + "\n" + line.1;
        } else {
            let mut chars: Vec<char> = line.1.chars().collect();

            if chars[smudge.0] == '#' {
                chars[smudge.0] = '.';
            } else {
                if chars[smudge.0] == '.' {
                    chars[smudge.0] = '#';
                }
            }
            let new_line: String = chars.into_iter().collect();
            new_str = new_str + "\n" + &new_line;
        }
    }

    new_str
}

fn get_line_smudge(lines: &Vec<&str>) -> Option<(usize, usize)> {
    let mut mirroring: Vec<Vec<usize>> = vec![];

    for line_str in lines {
        let line: Vec<char> = line_str.chars().collect();

        let indexes = mirror_line_indexes((0..lines[0].len()).collect(), &line);

        println!("Line {:?} indexes {:?}", line, indexes);
        mirroring.push(indexes);
    }

    let mut mirror_indexes: Vec<&usize> = mirroring.iter().flatten().collect();
    mirror_indexes.sort();
    mirror_indexes.dedup();

    let mut solution: Option<(usize, usize)> = None;

    for mr_in in &mirror_indexes {
        let mut do_not_have_it: Vec<usize> = vec![];
        for (line_nr, line_str) in lines.iter().enumerate() {
            let line: Vec<char> = line_str.chars().collect();

            if !mirror_on_line_char(**mr_in, &line) {
                do_not_have_it.push(line_nr);
            }

            // do_not_have_it.append(&mut errors_on_line_char(**mr_in, &line).unwrap());

        }

        do_not_have_it.sort();

        println!("for char {} it is line {:?}", mr_in, do_not_have_it);
        if do_not_have_it.len() == 1 && solution.is_none() {
            solution = Some((do_not_have_it[0], **mr_in));
        }
        if (do_not_have_it.len() == 0) && solution.is_none() {
            solution = Some((1000, **mr_in));
        }
    }

    println!("for char {:?}", solution);

    solution
}

fn get_column_smudge(lines: &Vec<&str>) -> Option<(usize, usize)> {
    let mut mirroring: Vec<Vec<usize>> = vec![];

    for column in 0..lines[0].len() {
        let line: Vec<char> = lines.iter().filter_map(|f| f.chars().nth(column)).collect();

        let indexes = mirror_line_indexes((0..lines[0].len()).collect(), &line);

        println!("Line {:?} wrongs {:?}", line, indexes);
        mirroring.push(indexes);
    }

    let mut mirror_indexes: Vec<&usize> = mirroring.iter().flatten().collect();
    mirror_indexes.sort();
    mirror_indexes.dedup();

    let mut solution: Option<(usize, usize)> = None;

    for mr_in in &mirror_indexes {
        let mut do_not_have_it: Vec<usize> = vec![];
        for column in 0..lines[0].len() {
            let line: Vec<char> = lines.iter().filter_map(|f| f.chars().nth(column)).collect();

            if !mirror_on_line_char(**mr_in, &line) {
                do_not_have_it.push(column);
            }

            // println!("Line {:?}",errors_on_line_char(**mr_in, &line));
            // do_not_have_it.append(&mut errors_on_line_char(**mr_in, &line).unwrap());
        }

        do_not_have_it.sort();

        println!("for char {} it is line {:?}", mr_in, do_not_have_it);
        if (do_not_have_it.len() == 1) && solution.is_none() {
            solution = Some((do_not_have_it[0], **mr_in));
        }
        if (do_not_have_it.len() == 0) && solution.is_none() {
            solution = Some((1000, **mr_in));
        }
    }
    println!("for char {:?}", solution);

    solution
}

fn errors_on_line_char(line_index: usize, line: &Vec<char>) -> Option<Vec<usize>> {
    if line_index >= line.len() || line_index == 0 {
        return None;
    }
    let left_side_rev: Vec<char> = (&line[0..line_index])
        .iter()
        .rev()
        .map(|f| f.clone())
        .collect();
    let right_side: Vec<char> = (&line[line_index..]).iter().map(|f| f.clone()).collect();

    // println!("         Lin\n          {:?}\n          {:?}",left_side_rev,right_side);

    let mut wrongs: Vec<usize> = vec![];

    for index in 0..(left_side_rev.len().min(right_side.len())) {
        if left_side_rev[index.clone()] != right_side[index] {
            wrongs.push(index);
        }
    }

    Some(wrongs)
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

    for index in 0..(left_side_rev.len().min(right_side.len())) {
        if left_side_rev[index.clone()] != right_side[index] {
            return false;
        }
    }

    true
}
