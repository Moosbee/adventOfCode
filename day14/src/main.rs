use std::collections::HashMap;
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

    let mut cycled: Vec<Vec<char>> = cycle_rows(&input_rows);

    let mut score_count: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut circle_size = 1000000001;
    let mut last_index = 1000000001;
    let mut shuffle_count = 0;

    for i in 1..(1000000001) {
        shuffle_count = shuffle_count + 1;
        cycled = cycle_rows(&cycled);
        // println!("\nComp {}", i);
        // print_rows(&cycled);
        let score = calc_score(&cycled);

        let lst_index = score_count.get(&cycled);

        if lst_index.is_some() {
            circle_size = i - lst_index.unwrap();
            last_index = i;
            println!(
                "Loop At index {} last {} circle_size {} score {}",
                i,
                lst_index.unwrap(),
                circle_size,
                score
            );
            print_rows(&cycled);
            break;
        } else {
            score_count.insert(cycled.clone(), i);
        }
    }

    let remaining = (1000000000 - last_index) % circle_size;

    println!("Remaining: {} size {}", remaining, circle_size);

    //   for i in 1..(circle_size) {
    //     cycled = cycle_rows(&cycled);
    //     // println!("\nComp {}", i);
    //     // print_rows(&cycled);
    //     let score = calc_score(&cycled);

    //     println!(
    //         "Loop At index {} circle_size {} score {}",
    //         i, circle_size, score
    //     );
    //     print_rows(&cycled);
    // }
    println!("Remaining: {} size {}", remaining, circle_size);

    for i in 0..(remaining-1) {
        shuffle_count = shuffle_count + 1;

        cycled = cycle_rows(&cycled);
        // println!("\nComp {}", i);
        // print_rows(&cycled);
        let score = calc_score(&cycled);

        println!(
            "Loop At index {} circle_size {} score {}",
            i, circle_size, score
        );
        print_rows(&cycled);
    }

    println!(
        "Rows: {:?}, Compressed: {:?} cycles: {} times shuffled: {}",
        input_rows.len(),
        cycled.len(),
        circle_size,
        shuffle_count
    );

    println!("Part 2: {}", calc_score(&cycled));

    println!("Took {:?}", start.elapsed())
}

fn calc_score(rows_st: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for comp_row in rows_st {
        for comp_char in comp_row.iter().enumerate() {
            if *comp_char.1 == 'O' {
                let score = comp_row.len() - comp_char.0;
                // println!(
                //     "Index: {} char {} score: {}",
                //     comp_char.0, comp_char.1, score
                // );
                total = total + score;
            }
        }
    }
    total
}

fn cycle_rows(rows_st: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut compressed_rows = rows_st.clone();
    compressed_rows = compress_up_all(&compressed_rows);
    // println!("Comp north");
    // print_rows(&compressed_rows);

    compressed_rows = change_rows(&compress_up_all(&change_rows(&compressed_rows)));
    // println!("Comp west");
    // print_rows(&compressed_rows);

    compressed_rows = compress_down_all(&compressed_rows);
    // println!("Comp south");
    // print_rows(&compressed_rows);

    compressed_rows = change_rows(&compress_down_all(&change_rows(&compressed_rows)));
    // println!("Comp east");
    // print_rows(&compressed_rows);
    compressed_rows
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

fn compress_down_all(rows_st: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rows = rows_st.clone();
    for row in &mut rows {
        row.reverse();
        compress_left(row);
        row.reverse();
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
