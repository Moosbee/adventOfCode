use std::fs;

use std::time::Instant;

use colored::Colorize;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let pipes: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    let mut count = 0;

    let mut current_pipe: PipePos = PipePos {
        pipe_type: 'S',
        pos: (0, 0),
    };

    let mut scores: Vec<Vec<i32>> = pipes
        .iter()
        .enumerate()
        .map(|line| {
            line.1
                .iter()
                .enumerate()
                .map(|f| {
                    if f.1 == &'S' {
                        current_pipe = PipePos {
                            pipe_type: f.1.clone(),
                            pos: (line.0, f.0),
                        };
                    }
                    0
                })
                .collect()
        })
        .collect();

    let mut last_pipe = current_pipe;

    loop {
        let (first_pipe, second_pipe) = get_next_pipe(current_pipe, &pipes);

        // println!(
        //     "Pipe {:?} next to {:?} and {:?} was {:?}",
        //     current_pipe, first_pipe, second_pipe, last_pipe
        // );

        if scores[current_pipe.pos.0][current_pipe.pos.1] != 0 {
            println!("Problem");
        }

        scores[current_pipe.pos.0][current_pipe.pos.1] = count;

        if first_pipe.is_some() && first_pipe.unwrap().pos != last_pipe.pos {
            println!(
                "Pipe from {:?} to {:?} because {} {:?} != {:?}",
                current_pipe,
                first_pipe.unwrap(),
                first_pipe.is_some(),
                first_pipe.unwrap().pos,
                last_pipe.pos
            );
            last_pipe = current_pipe;
            current_pipe = first_pipe.unwrap();
        } else if second_pipe.is_some() && second_pipe.unwrap().pos != last_pipe.pos {
            println!(
                "Pipe from {:?} to {:?} because {} {:?} != {:?}",
                current_pipe,
                second_pipe.unwrap(),
                second_pipe.is_some(),
                second_pipe.unwrap().pos,
                last_pipe.pos
            );
            last_pipe = current_pipe;
            current_pipe = second_pipe.unwrap();
        } else {
            println!(
                "Pipe from {:?} to {:?} {:?} is wrong",
                current_pipe, first_pipe, second_pipe,
            );
        }

        // print_board(&pipes, &scores);

        count = count + 1;
        if current_pipe.pipe_type == 'S' || count > 640000 {
            break;
        }
    }

    print_board(&pipes, &scores);

    let mut in_count = 0;

    for pipe_line in pipes.iter().enumerate() {
        let mut inside = false;
        let mut last_pipe = ' ';

        for pipe in pipe_line.1.iter().enumerate() {
            let score = scores[pipe_line.0][pipe.0];
            let pipe_type = pipe.1;

            if score == 0 {
                if inside {
                    scores[pipe_line.0][pipe.0] = -1;
                    in_count = in_count + 1;
                }
            } else if score == -1 {
            } else {
                // ─│┌┐└┘
                match pipe_type {
                    '─' => {}
                    '│' => {
                        inside = !inside;
                    }
                    '┌' => {
                        inside = !inside;
                        last_pipe = '┌';
                    }
                    '┐' => {
                        match last_pipe {
                            '┌' => {
                                inside = !inside;
                            }
                            '└' => {}
                            _ => {
                                // inside = !inside;
                            }
                        }
                        last_pipe = '.';
                    }
                    '└' => {
                        inside = !inside;
                        last_pipe = '└';
                    }
                    '┘' => {
                        match last_pipe {
                            '┌' => {}
                            '└' => {
                                inside = !inside;
                            }
                            _ => {
                                // inside = !inside;
                            }
                        }
                        last_pipe = '.';
                    }
                    'S' => {
                        inside = !inside;
                        last_pipe = 'S';
                    }
                    _ => {}
                }
            }
        }
    }

    print_board(&pipes, &scores);

    println!(
        "Solution: {} ({}) second: {} took {:?}",
        count / 2,
        count,
        in_count,
        start.elapsed()
    );
}

#[derive(Debug, Clone, Copy)]
pub struct PipePos {
    pipe_type: char,
    pos: (usize, usize),
}

fn get_next_pipe(
    current_pos: PipePos,
    pipes: &Vec<Vec<char>>,
) -> (Option<PipePos>, Option<PipePos>) {
    let pipe_type = &pipes[current_pos.pos.0][current_pos.pos.1];

    let up_pipe: Option<PipePos>;
    if current_pos.pos.0 != 0 {
        let maybe_part = pipes[current_pos.pos.0 - 1][current_pos.pos.1];
        if maybe_part != '.' {
            up_pipe = Some(PipePos {
                pipe_type: maybe_part,
                pos: (current_pos.pos.0 - 1, current_pos.pos.1),
            });
        } else {
            up_pipe = None;
        }
    } else {
        up_pipe = None;
    }

    let down_pipe: Option<PipePos>;
    if current_pos.pos.0 != pipes.len() - 1 {
        let maybe_part = pipes[current_pos.pos.0 + 1][current_pos.pos.1];
        if maybe_part != '.' {
            down_pipe = Some(PipePos {
                pipe_type: maybe_part,
                pos: (current_pos.pos.0 + 1, current_pos.pos.1),
            });
        } else {
            down_pipe = None;
        }
    } else {
        down_pipe = None;
    }

    let left_pipe: Option<PipePos>;
    if current_pos.pos.1 != 0 {
        let maybe_part = pipes[current_pos.pos.0][current_pos.pos.1 - 1];
        if maybe_part != '.' {
            left_pipe = Some(PipePos {
                pipe_type: maybe_part,
                pos: (current_pos.pos.0, current_pos.pos.1 - 1),
            });
        } else {
            left_pipe = None;
        }
    } else {
        left_pipe = None;
    }

    let right_pipe: Option<PipePos>;
    if current_pos.pos.1 != pipes.len() - 1 {
        let maybe_part = pipes[current_pos.pos.0][current_pos.pos.1 + 1];
        if maybe_part != '.' {
            right_pipe = Some(PipePos {
                pipe_type: maybe_part,
                pos: (current_pos.pos.0, current_pos.pos.1 + 1),
            });
        } else {
            right_pipe = None;
        }
    } else {
        right_pipe = None;
    }

    // println!(
    //     "Pipes from {:?} be up: {:?} left: {:?} dow: {:?} right: {:?}",
    //     current_pos, up_pipe, left_pipe, down_pipe, right_pipe
    // );

    match pipe_type {
        'S' => {
            let mut tiles: [Option<PipePos>; 2] = [None, None];

            println!(
                "Start from {:?} to up: {:?} left: {:?} dow: {:?} right: {:?}",
                current_pos, up_pipe, left_pipe, down_pipe, right_pipe
            );

            if up_pipe.is_some() {
                let prt = up_pipe.unwrap();
                if prt.pipe_type == '┐' || prt.pipe_type == '┌' || prt.pipe_type == '│' {
                    if tiles[0].is_none() {
                        tiles[0] = up_pipe;
                    } else if tiles[1].is_none() {
                        tiles[0] = up_pipe;
                    }
                }
            }
            if down_pipe.is_some() {
                let prt = down_pipe.unwrap();
                if prt.pipe_type == '┘' || prt.pipe_type == '└' || prt.pipe_type == '│' {
                    if tiles[0].is_none() {
                        tiles[0] = down_pipe;
                    } else if tiles[1].is_none() {
                        tiles[0] = down_pipe;
                    }
                }
            }
            if right_pipe.is_some() {
                let prt = right_pipe.unwrap();
                if prt.pipe_type == '┘' || prt.pipe_type == '┐' || prt.pipe_type == '─' {
                    if tiles[0].is_none() {
                        tiles[0] = right_pipe;
                    } else if tiles[1].is_none() {
                        tiles[0] = right_pipe;
                    }
                }
            }
            if left_pipe.is_some() {
                let prt = left_pipe.unwrap();
                if prt.pipe_type == '└' || prt.pipe_type == '┌' || prt.pipe_type == '─' {
                    if tiles[0].is_none() {
                        tiles[0] = left_pipe;
                    } else if tiles[1].is_none() {
                        tiles[0] = left_pipe;
                    }
                }
            }

            (tiles[0], tiles[1])
        }
        '┐' => (down_pipe, left_pipe),
        '┌' => (down_pipe, right_pipe),
        '─' => (right_pipe, left_pipe),
        '│' => (up_pipe, down_pipe),
        '┘' => (up_pipe, left_pipe),
        '└' => (up_pipe, right_pipe),
        _ => (None, None),
    }
}

fn print_board(pipes: &Vec<Vec<char>>, scores: &Vec<Vec<i32>>) {
    for pipe_line in pipes.iter().enumerate() {
        print!("          ");
        for pipe in pipe_line.1.iter().enumerate() {
            // print!("{number:>width$}", number = pipe, width = 3);
            let score = scores[pipe_line.0][pipe.0];
            if score == 0 {
                print!("{}", pipe.1);
            } else if score == -1 {
                print!("{}", pipe.1.to_string().green());
            } else {
                print!("{}", pipe.1.to_string().yellow());
            }
        }
        println!();
    }
    println!();
    println!();
}
