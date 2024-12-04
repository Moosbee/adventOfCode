use std::{fs, time::Instant};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let letters = input_lines
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut total_xmas = 0;
    let mut total_x_mas = 0;

    for l in 0..letters.len() {
        for c in 0..letters[l].len() {
            let letter = letters[l][c];
            if letter == 'X' {
                let xmas = is_xmas(&letters, l, c);
                total_xmas = total_xmas + xmas;
                println!("{} {} {} {} {}", letter, l, c, xmas, total_xmas);
            }
            if letter == 'A' {
                let x_mas = is_x_mas(&letters, l, c);
                total_x_mas = total_x_mas + x_mas as u32;
                println!("{} {} {} {} {}", letter, l, c, x_mas as u32, total_x_mas);
            }
        }
    }

    println!(
        "Total XMAS {} Total X-MAS {} took {:?}",
        total_xmas,
        total_x_mas,
        start.elapsed()
    )
}

fn is_xmas(lines: &Vec<Vec<char>>, l: usize, c: usize) -> u32 {
    let get_char = |r: isize, col: isize| -> Option<char> {
        if r >= 0 && col >= 0 {
            lines
                .get(r as usize)
                .map(|colon| colon.get(col as usize))
                .flatten()
                .cloned()
        } else {
            None
        }
    };

    let middel = get_char(l as isize, c as isize).unwrap(); // should be X and possible

    let right_m = get_char(l as isize, c as isize + 1).unwrap_or(' ');
    let left_m = get_char(l as isize, c as isize - 1).unwrap_or(' ');
    let top_m = get_char(l as isize - 1, c as isize).unwrap_or(' ');
    let bottom_m = get_char(l as isize + 1, c as isize).unwrap_or(' ');
    let top_right_m = get_char(l as isize - 1, c as isize + 1).unwrap_or(' ');
    let top_left_m = get_char(l as isize - 1, c as isize - 1).unwrap_or(' ');
    let bottom_right_m = get_char(l as isize + 1, c as isize + 1).unwrap_or(' ');
    let bottom_left_m = get_char(l as isize + 1, c as isize - 1).unwrap_or(' ');

    let right_a = get_char(l as isize, c as isize + 2).unwrap_or(' ');
    let left_a = get_char(l as isize, c as isize - 2).unwrap_or(' ');
    let top_a = get_char(l as isize - 2, c as isize).unwrap_or(' ');
    let bottom_a = get_char(l as isize + 2, c as isize).unwrap_or(' ');
    let top_right_a = get_char(l as isize - 2, c as isize + 2).unwrap_or(' ');
    let top_left_a = get_char(l as isize - 2, c as isize - 2).unwrap_or(' ');
    let bottom_right_a = get_char(l as isize + 2, c as isize + 2).unwrap_or(' ');
    let bottom_left_a = get_char(l as isize + 2, c as isize - 2).unwrap_or(' ');

    let right_s = get_char(l as isize, c as isize + 3).unwrap_or(' ');
    let left_s = get_char(l as isize, c as isize - 3).unwrap_or(' ');
    let top_s = get_char(l as isize - 3, c as isize).unwrap_or(' ');
    let bottom_s = get_char(l as isize + 3, c as isize).unwrap_or(' ');
    let top_right_s = get_char(l as isize - 3, c as isize + 3).unwrap_or(' ');
    let top_left_s = get_char(l as isize - 3, c as isize - 3).unwrap_or(' ');
    let bottom_right_s = get_char(l as isize + 3, c as isize + 3).unwrap_or(' ');
    let bottom_left_s = get_char(l as isize + 3, c as isize - 3).unwrap_or(' ');

    let right_xmas = middel == 'X' && right_m == 'M' && right_a == 'A' && right_s == 'S';
    let left_xmas = middel == 'X' && left_m == 'M' && left_a == 'A' && left_s == 'S';
    let top_xmas = middel == 'X' && top_m == 'M' && top_a == 'A' && top_s == 'S';
    let bottom_xmas = middel == 'X' && bottom_m == 'M' && bottom_a == 'A' && bottom_s == 'S';
    let top_right_xmas =
        middel == 'X' && top_right_m == 'M' && top_right_a == 'A' && top_right_s == 'S';
    let top_left_xmas =
        middel == 'X' && top_left_m == 'M' && top_left_a == 'A' && top_left_s == 'S';
    let bottom_right_xmas =
        middel == 'X' && bottom_right_m == 'M' && bottom_right_a == 'A' && bottom_right_s == 'S';
    let bottom_left_xmas =
        middel == 'X' && bottom_left_m == 'M' && bottom_left_a == 'A' && bottom_left_s == 'S';

    let count = right_xmas as u32
        + left_xmas as u32
        + top_xmas as u32
        + bottom_xmas as u32
        + top_right_xmas as u32
        + top_left_xmas as u32
        + bottom_right_xmas as u32
        + bottom_left_xmas as u32;
    count
}

fn is_x_mas(lines: &Vec<Vec<char>>, l: usize, c: usize) -> bool {
    let get_char = |r: isize, col: isize| -> Option<char> {
        if r >= 0 && col >= 0 {
            lines
                .get(r as usize)
                .map(|colon| colon.get(col as usize))
                .flatten()
                .cloned()
        } else {
            None
        }
    };

    let middel = get_char(l as isize, c as isize).unwrap(); // should be X and possible

    let top_left = get_char(l as isize - 1, c as isize - 1).unwrap_or(' ');
    let bottom_right = get_char(l as isize + 1, c as isize + 1).unwrap_or(' ');
    let bottom_left = get_char(l as isize + 1, c as isize - 1).unwrap_or(' ');
    let top_right = get_char(l as isize - 1, c as isize + 1).unwrap_or(' ');

    let dig_bottom_left_to_top_right = bottom_left == 'M' && middel == 'A' && top_right == 'S';
    let dig_top_left_to_bottom_right = top_left == 'M' && middel == 'A' && bottom_right == 'S';
    let dig_bottom_right_to_top_left = bottom_right == 'M' && middel == 'A' && top_left == 'S';
    let dig_top_right_to_bottom_left = top_right == 'M' && middel == 'A' && bottom_left == 'S';

    let dig_1 = dig_bottom_left_to_top_right || dig_top_right_to_bottom_left;
    let dig_2 = dig_top_left_to_bottom_right || dig_bottom_right_to_top_left;

    dig_1 && dig_2
}
