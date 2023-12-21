use std::collections::HashMap;
use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let garden: Vec<Vec<char>> = input_lines.map(|f| f.chars().collect()).collect();

    print_garden(&garden);

    // for _index in -100..100 {
    //     for _index_r in -100..100 {
    //         can_go((_index, _index_r), &garden);
    //     }
    // }

    // let mut steps: Vec<(usize, usize)> = vec![(
    //     (garden.len() / 2).try_into().unwrap(),
    //     (garden[0].len() / 2).try_into().unwrap(),
    // )];
    let mut steps: HashMap<(i32, i32), bool> = HashMap::new();
    steps.insert(
        (
            (garden.len() / 2).try_into().unwrap(),
            (garden[0].len() / 2).try_into().unwrap(),
        ),
        true,
    );

    // println!(
    //     "Char at {:?} is {}",
    //     steps[0], garden[steps[0].0][steps[0].1]
    // );

    //     26501365

    // 26501365*1,13

    // 29946542,45

    // 5472,3434148452

    let mut all_steps: HashMap<(i32, i32), bool> = HashMap::new();

    for _i in 0..(10000 as i32) {
        // 5148 / 2574 / 20
        let mut new_steps: HashMap<(i32, i32), bool> = HashMap::new();

        for step in steps {
            if can_go((step.0 .0 - 1, step.0 .1), &garden) {
                new_steps.insert((step.0 .0 - 1, step.0 .1), true);
                all_steps.insert((step.0 .0 - 1, step.0 .1), true);
            }
            if can_go((step.0 .0 + 1, step.0 .1), &garden) {
                new_steps.insert((step.0 .0 + 1, step.0 .1), true);
                all_steps.insert((step.0 .0 + 1, step.0 .1), true);
            }
            if can_go((step.0 .0, step.0 .1 - 1), &garden) {
                new_steps.insert((step.0 .0, step.0 .1 - 1), true);
                all_steps.insert((step.0 .0, step.0 .1 - 1), true);
            }
            if can_go((step.0 .0, step.0 .1 + 1), &garden) {
                new_steps.insert((step.0 .0, step.0 .1 + 1), true);
                all_steps.insert((step.0 .0, step.0 .1 + 1), true);
            }
        }
        steps = new_steps;

        println!(
            "Step {} count {} max {} diff {} all {} all half {} diff {}",
            _i + 1,
            steps.len(),
            (_i + 1).pow(2),
            ((_i + 1).pow(2) as f64) / steps.len() as f64,
            all_steps.len(),
            all_steps.len() / 2,
            steps.len() as i32 - (all_steps.len() / 2) as i32
        );
        if steps.len() == 26501365 {
            break;
        }
    }

    // println!(
    //     "Start of graph {:?} graph\n{:?} \n can be displayed in https://viz-js.com/",
    //     start_point,
    //     Dot::new(&plots)
    // );

    println!("Solution: {} Took {:?}", steps.len(), start.elapsed())
}

fn can_go(pos: (i32, i32), garden: &Vec<Vec<char>>) -> bool {
    let line_index_num = pos.0 % garden.len() as i32;
    let line_index = if line_index_num < 0 {
        garden.len() - line_index_num.abs() as usize
    } else {
        line_index_num.abs() as usize
    };
    let row_index_num = pos.1 % garden[line_index].len() as i32;

    let row_index = if row_index_num < 0 {
        garden[line_index].len() - row_index_num.abs() as usize
    } else {
        row_index_num.abs() as usize
    };
    let chr = garden[line_index][row_index];
    // println!(
    //     "Char at pos {:?} index {} {} is {}",
    //     pos, line_index, row_index, chr
    // );

    chr != '#'
}

fn print_garden(garden: &Vec<Vec<char>>) {
    for garden_line in garden {
        print!("  ");

        for plot in garden_line {
            print!("{}", plot)
        }
        println!()
    }
}
