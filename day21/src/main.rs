use std::fs;

use std::time::Instant;

use colored::Colorize;

#[derive(Debug, Clone, Copy)]
struct Pos {
    p_type: char,
    state: bool,
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut garden: Vec<Vec<Pos>> = input_lines
        .clone()
        .map(|f| {
            f.chars()
                .map(|l| Pos {
                    p_type: l,
                    state: l == 'S',
                })
                .collect()
        })
        .collect();

    let start_garden: Vec<Vec<Pos>> = input_lines
        .map(|f| {
            f.chars()
                .map(|l| Pos {
                    p_type: l,
                    state: false,
                })
                .collect()
        })
        .collect();

    print_garden(&garden);

    for _i in 0..64 {
        let mut new_garden: Vec<Vec<Pos>> = start_garden.clone();
        let mut step_count = 0;
        for (line_index, garden_line) in garden.iter().enumerate() {
            for (plot_index, plot) in garden_line.iter().enumerate() {
                if plot.p_type != '#' && plot.state {
                    if line_index != 0 {
                        if !new_garden[line_index - 1][plot_index].state
                            && new_garden[line_index - 1][plot_index].p_type != '#'
                        {
                            step_count = step_count + 1;
                        }
                        new_garden[line_index - 1][plot_index].state = true; // UP
                    }
                    if line_index + 1 < new_garden.len() {
                        if !new_garden[line_index + 1][plot_index].state
                            && new_garden[line_index + 1][plot_index].p_type != '#'
                        {
                            step_count = step_count + 1;
                        }
                        new_garden[line_index + 1][plot_index].state = true; // DOWN
                    }
                    if plot_index != 0 {
                        if !new_garden[line_index][plot_index - 1].state
                            && new_garden[line_index][plot_index - 1].p_type != '#'
                        {
                            step_count = step_count + 1;
                        }
                        new_garden[line_index][plot_index - 1].state = true; // LEFT
                    }
                    if plot_index + 1 < new_garden[line_index].len() {
                        if !new_garden[line_index][plot_index + 1].state
                            && new_garden[line_index][plot_index + 1].p_type != '#'
                        {
                            step_count = step_count + 1;
                        }
                        new_garden[line_index][plot_index + 1].state = true; // RIGHT
                    }
                }
            }
        }
        garden = new_garden;
        println!("Step {} count {}", _i + 1, step_count);
        print_garden(&garden);
    }

    // println!(
    //     "Start of graph {:?} graph\n{:?} \n can be displayed in https://viz-js.com/",
    //     start_point,
    //     Dot::new(&plots)
    // );

    println!(
        "Solution: {} Took {:?}",
        calc_steps(&garden),
        start.elapsed()
    )
}

fn print_garden(garden: &Vec<Vec<Pos>>) {
    for garden_line in garden {
        print!("  ");

        for plot in garden_line {
            if plot.state {
                if plot.p_type != '#' {
                    print!("{}", 'O'.to_string().green())
                } else {
                    print!("{}", plot.p_type.to_string().yellow())
                }
            } else {
                print!("{}", plot.p_type)
            }
        }
        println!()
    }
}

fn calc_steps(garden: &Vec<Vec<Pos>>) -> i32 {
    let mut count = 0;
    for garden_line in garden {
        for plot in garden_line {
            if plot.state {
                if plot.p_type != '#' {
                    count += 1;
                }
            }
        }
    }
    count
}
