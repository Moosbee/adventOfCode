use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines: std::str::Lines<'_> = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let char_vec: Vec<Vec<char>> = input_lines.map(|f| f.chars().collect()).collect();

    println!("Col");
    print_column(&char_vec);

    let input_rows: Vec<Vec<char>> = change_rows(&char_vec);
    println!("Row");
    print_rows(&input_rows);

    let compressed = compress_up_all(&input_rows);
    println!("Comp");
    print_rows(&compressed);

    println!(
        "Rows: {:?}, Compressed: {:?}",
        input_rows.len(),
        compressed.len()
    );

    let mut total = 0;

    for comp_row in compressed {
        for comp_char in comp_row.iter().enumerate() {
            if *comp_char.1 == 'O' {
                let score = comp_row.len() - comp_char.0;
                println!(
                    "Index: {} char {} score: {}",
                    comp_char.0, comp_char.1, score
                );
                total = total + score;
            }
        }
    }

    println!("Part 1: {}", total);

    println!("Took {:?}", start.elapsed())
}

fn print_column(columns: &Vec<Vec<char>>) {
    for column in columns {
        for chr in column {
            print!("{}", chr);
        }
        println!();
    }
}

fn print_rows(rows_st: &Vec<Vec<char>>) {
    let column_again = change_rows(rows_st);
    print_column(&column_again)
}

fn change_rows(input_lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input_rows: Vec<Vec<char>> = vec![];

    for index in 0..input_lines[0].len() {
        let mut row: Vec<char> = vec![];
        for line in input_lines.clone() {
            row.push(line[index]);
        }
        input_rows.push(row);
    }

    input_rows
}

fn compress_up_all(rows_st: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rows = rows_st.clone();
    for row in &mut rows {
        compress_left(row);
    }
    rows
}

fn compress_left(row: &mut Vec<char>) {
    for index in 0..row.len() {
        if row[index] == 'O' {
            row[index] = '.';
            for jaded in (0..(index + 1)).rev() {
                if row[jaded] != '.' {
                    row[jaded + 1] = 'O';
                    break;
                } else if jaded == 0 {
                    row[jaded] = 'O';
                    break;
                }
            }
        }
    }
}
