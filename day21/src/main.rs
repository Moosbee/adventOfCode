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
    let input =
        fs::read_to_string("./test_2_input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let garden: Vec<Vec<Pos>> = input_lines
        .map(|f| {
            f.chars()
                .map(|l| Pos {
                    p_type: if l == '#' { '#' } else { '.' },
                    state: false,
                })
                .collect()
        })
        .collect();

    let from_up_left: Vec<usize> = get_stats(garden.clone(), (0, 0), garden.len() * 2);
    let from_up_middle: Vec<usize> =
        get_stats(garden.clone(), (0, garden[0].len() / 2), garden.len() * 2);
    let from_up_right: Vec<usize> =
        get_stats(garden.clone(), (0, garden[0].len() - 1), garden.len() * 2);
    let from_middle_left: Vec<usize> =
        get_stats(garden.clone(), (garden.len() / 2, 0), garden.len() * 2);
    let from_middle_middle: Vec<usize> = get_stats(
        garden.clone(),
        (garden.len() / 2, garden[0].len() / 2),
        garden.len() * 2,
    );
    let from_middle_right: Vec<usize> = get_stats(
        garden.clone(),
        (garden.len() / 2, garden[0].len() - 1),
        garden.len() * 2,
    );
    let from_bottom_left: Vec<usize> =
        get_stats(garden.clone(), (garden.len() - 1, 0), garden.len() * 2);
    let from_bottom_middle: Vec<usize> = get_stats(
        garden.clone(),
        (garden.len() - 1, garden[0].len() / 2),
        garden.len() * 2,
    );
    let from_bottom_right: Vec<usize> = get_stats(
        garden.clone(),
        (garden.len() - 1, garden[0].len() - 1),
        garden.len() * 2,
    );

    // println!("from_up_left: {:?}", from_up_left);
    // println!("from_up_middle: {:?}", from_up_middle);
    // println!("from_up_right: {:?}", from_up_right);
    // println!("from_middle_left: {:?}", from_middle_left);
    // println!("from_middle_middle: {:?}", from_middle_middle);
    // println!("from_middle_right: {:?}", from_middle_right);
    // println!("from_bottom_left: {:?}", from_bottom_left);
    // println!("from_bottom_middle: {:?}", from_bottom_middle);
    // println!("from_bottom_right: {:?}", from_bottom_right);

    println!();
    println!();
    println!();

    // test(
    //     &garden,
    //     &from_up_left,
    //     &from_up_middle,
    //     &from_up_right,
    //     &from_middle_left,
    //     &from_middle_middle,
    //     &from_middle_right,
    //     &from_bottom_left,
    //     &from_bottom_middle,
    //     &from_bottom_right,
    // );

    let numbers = vec![38, 64, 86, 140, 141, 200, 1002, 4178]; //, 3396, 658, 2687, 1716, 3910, 4178, 4678, 3887, 3432,
                                                               // ];

    for number in numbers {
        let erg = calc_stats_of_step(
            &garden,
            &from_up_left,
            &from_up_middle,
            &from_up_right,
            &from_middle_left,
            &from_middle_middle,
            &from_middle_right,
            &from_bottom_left,
            &from_bottom_middle,
            &from_bottom_right,
            number,
        );
        let text;
        if erg == ((number + 1) as i32).pow(2).try_into().unwrap() {
            text = "ja#######".green()
        } else {
            text = "na######".red()
        }
        println!(
            "For Number {} erg {:?} is equal to {} {}",
            number,
            erg,
            ((number + 1) as i32).pow(2),
            text
        );
    }

    println!(
        "Solution: {} Took {:?}",
        from_middle_middle.len(),
        start.elapsed()
    );
    print_garden(&garden);
    test(
        &garden,
        &from_up_left,
        &from_up_middle,
        &from_up_right,
        &from_middle_left,
        &from_middle_middle,
        &from_middle_right,
        &from_bottom_left,
        &from_bottom_middle,
        &from_bottom_right,
    );
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

/// assumes from the middle
fn calc_stats_of_step(
    garden: &Vec<Vec<Pos>>,
    from_up_left: &Vec<usize>,
    from_up_middle: &Vec<usize>,
    from_up_right: &Vec<usize>,
    from_middle_left: &Vec<usize>,
    from_middle_middle: &Vec<usize>,
    from_middle_right: &Vec<usize>,
    from_bottom_left: &Vec<usize>,
    from_bottom_middle: &Vec<usize>,
    from_bottom_right: &Vec<usize>,
    step: usize,
) -> usize {
    let garden_size = garden.len();

    if step < garden_size / 2 {
        return from_middle_middle[step];
    }
    if step < garden_size {
        let next_step = step;

        let step_amount = next_step / garden_size;
        let step_rest = next_step % (garden_size / 2);

        let middle_middle = from_middle_middle[step];
        let up_middle = from_up_middle[step_rest - 1];
        let middle_left = from_middle_left[step_rest - 1];
        let middle_right = from_middle_right[step_rest - 1];
        let bottom_middle = from_bottom_middle[step_rest - 1];

        let dot_ja_count = (middle_middle) + up_middle + middle_left + middle_right + bottom_middle;

        println!(
            "Set Step {} size {} passt {} rest {} ja {}",
            step, garden_size, step_amount, step_rest, dot_ja_count
        );

        return dot_ja_count;
    }

    if step < (garden_size + garden_size / 2) {
        let next_step = step;

        let step_amount = next_step / garden_size;
        let step_rest = next_step % garden_size;
        let step_rest_long = (next_step % garden_size) + (garden_size / 2);
        let gerade = step % 2;
        let middle_middle = from_middle_middle[from_middle_middle.len() - 1 - gerade];
        let middle_left = from_middle_left[step_rest_long];
        let middle_right = from_middle_right[step_rest_long];
        let bottom_middle = from_bottom_middle[step_rest_long];
        let up_middle = from_up_middle[step_rest_long];

        let up_left = from_up_left[step_rest - 1];
        let up_right = from_up_right[step_rest - 1];
        let bottom_left = from_bottom_left[step_rest - 1];
        let bottom_right = from_bottom_right[step_rest - 1];

        let dot_ja_count = middle_middle// * step_amount * step_amount
            + up_middle
            + middle_left
            + middle_right
            + bottom_middle
            + up_left
            + up_right
            + bottom_left
            + bottom_right;

        println!(
            "L Step {} size {} passt {} rest {} ja {}",
            step, garden_size, step_amount, step_rest, dot_ja_count
        );

        return dot_ja_count;
    }

    let next_step = step - ((garden_size - 1) / 2) - 1;
    // let next_step = step;

    let step_amount = next_step / garden_size;
    let step_rest = next_step % garden_size;
    let step_rest_bg = next_step % (garden_size * 2);

    let big_tr_index = if step_rest == step_rest_bg {
        0
    } else {
        step_rest_bg
    };
    let small_tr_index = step_rest;
    let big_middle_index = if step_rest == step_rest_bg {
        0
    } else {
        step_rest_bg
    };
    let small_middle = step_rest;

    let middle_index = from_middle_middle.len() - 1;

    let big_tr_count = step_amount;
    let small_tr_count = step_amount;
    let middle_count = step_amount;

    let middle_middle = from_middle_middle[middle_index];

    let small_up_middle = from_up_middle[small_middle];
    let small_middle_left = from_middle_left[small_middle];
    let small_middle_right = from_middle_right[small_middle];
    let small_bottom_middle = from_bottom_middle[small_middle];

    let big_up_middle = from_up_middle[big_middle_index];
    let big_middle_left = from_middle_left[big_middle_index];
    let big_middle_right = from_middle_right[big_middle_index];
    let big_bottom_middle = from_bottom_middle[big_middle_index];

    let small_up_left = from_up_left[small_tr_index];
    let small_up_right = from_up_right[small_tr_index];
    let small_bottom_left = from_bottom_left[small_tr_index];
    let small_bottom_right = from_bottom_right[small_tr_index];

    let big_up_left = from_up_left[big_tr_index];
    let big_up_right = from_up_right[big_tr_index];
    let big_bottom_left = from_bottom_left[big_tr_index];
    let big_bottom_right = from_bottom_right[big_tr_index];

    let dot_ja_count = (middle_middle * middle_count)
        + small_up_middle
        + small_middle_left
        + small_middle_right
        + small_bottom_middle
        + big_up_middle
        + big_middle_left
        + big_middle_right
        + big_bottom_middle
        + small_up_left * small_tr_count
        + small_up_right * small_tr_count
        + small_bottom_left * small_tr_count
        + small_bottom_right * small_tr_count
        + big_up_left * big_tr_count
        + big_up_right * big_tr_count
        + big_bottom_left * big_tr_count
        + big_bottom_right * big_tr_count;

    println!(
        "Debug Print:\n\
        next_step:          {},\n\
        garden_size:        {},\n\
        step:               {},\n\
        step_amount:        {},\n\
        step_rest:          {},\n\
        step_rest_bg:       {},\n\
        big_tr_index:       {},\n\
        small_tr_index:     {},\n\
        big_middle_index:   {},\n\
        small_middle:       {},\n\
        middle_index:       {},\n\
        big_tr_count:       {},\n\
        small_tr_count:     {},\n\
        middle_count:       {},\n\
        middle_middle:      {},\n\
        small_up_middle:    {},\n\
        small_middle_left:  {},\n\
        small_middle_right: {},\n\
        small_bottom_middle:{},\n\
        big_up_middle:      {},\n\
        big_middle_left:    {},\n\
        big_middle_right:   {},\n\
        big_bottom_middle:  {},\n\
        small_up_left:      {},\n\
        small_up_right:     {},\n\
        small_bottom_left:  {},\n\
        small_bottom_right: {},\n\
        big_up_left:        {},\n\
        big_up_right:       {},\n\
        big_bottom_left:    {},\n\
        big_bottom_right:   {},\n\
        dot_ja_count:       {}",
        next_step,
        garden_size,
        step,
        step_amount,
        step_rest,
        step_rest_bg,
        big_tr_index,
        small_tr_index,
        big_middle_index,
        small_middle,
        middle_index,
        big_tr_count,
        small_tr_count,
        middle_count,
        middle_middle,
        small_up_middle,
        small_middle_left,
        small_middle_right,
        small_bottom_middle,
        big_up_middle,
        big_middle_left,
        big_middle_right,
        big_bottom_middle,
        small_up_left,
        small_up_right,
        small_bottom_left,
        small_bottom_right,
        big_up_left,
        big_up_right,
        big_bottom_left,
        big_bottom_right,
        dot_ja_count
    );
    return dot_ja_count;
}

fn get_stats(start: Vec<Vec<Pos>>, start_i: (usize, usize), size: usize) -> Vec<usize> {
    let start_garden: Vec<Vec<Pos>> = start;

    let mut garden = start_garden.clone();

    garden[start_i.0][start_i.1].state = true;

    // print_garden(&garden);

    let mut stats: Vec<_> = vec![];
    stats.push(calc_steps(&start_garden, 0));

    for _i in 0..size {
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
        stats.push(calc_steps(&garden, _i));
        // println!("Step {} count {}", _i + 1, step_count);
        // print_garden(&garden);
    }
    // print_garden(&garden);

    stats
}

fn calc_steps(garden: &Vec<Vec<Pos>>, _step: usize) -> usize {
    let mut dot_ja_count = 0;
    let mut _dot_na_count = 0;
    let mut _raute_ja_count = 0;
    let mut _raute_na_count = 0;
    for garden_line in garden {
        for plot in garden_line {
            if plot.state {
                if plot.p_type == '.' {
                    dot_ja_count += 1;
                } else {
                    _raute_ja_count += 1;
                }
            } else {
                if plot.p_type == '.' {
                    _dot_na_count += 1;
                } else {
                    _raute_na_count += 1;
                }
            }
        }
    }
    dot_ja_count
}

fn test(
    garden: &Vec<Vec<Pos>>,
    from_up_left: &Vec<usize>,
    from_up_middle: &Vec<usize>,
    from_up_right: &Vec<usize>,
    from_middle_left: &Vec<usize>,
    from_middle_middle: &Vec<usize>,
    from_middle_right: &Vec<usize>,
    from_bottom_left: &Vec<usize>,
    from_bottom_middle: &Vec<usize>,
    from_bottom_right: &Vec<usize>,
) {
    let number_list: Vec<(usize, usize)> = vec![
        (38, 1343),
        (252, 56512),
        (412, 150446),
        (1489, 1960349),
        (2187, 4228130),
        (3019, 8055797),
    ];

    for number in number_list {
        let erg = calc_stats_of_step(
            garden,
            from_up_left,
            from_up_middle,
            from_up_right,
            from_middle_left,
            from_middle_middle,
            from_middle_right,
            from_bottom_left,
            from_bottom_middle,
            from_bottom_right,
            number.0,
        );
        println!(
            "For Number {} erg {:?} is equal to {}",
            number.0, erg, number.1
        );
        assert_eq!(erg, number.1)
    }
}
