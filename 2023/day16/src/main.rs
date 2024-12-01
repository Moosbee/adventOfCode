use std::fs;

use std::time::Instant;

use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

struct LightRay {
    direction: Direction,
    x_pos: usize,
    y_pos: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mirrors: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();

    let energized = calc_illumination(
        &mirrors,
        vec![LightRay {
            direction: Direction::RIGHT,
            x_pos: 0,
            y_pos: 0,
        }],
        false,
    );

    let max_height: usize = mirrors.len();
    let max_width: usize = mirrors.last().unwrap().len();

    let mut all_starts: Vec<LightRay> = vec![];

    for height in 0..max_height {
        all_starts.push(LightRay {
            direction: Direction::RIGHT,
            x_pos: 0,
            y_pos: height,
        });
        all_starts.push(LightRay {
            direction: Direction::LEFT,
            x_pos: max_width - 1,
            y_pos: height,
        });
    }

    for with in 0..max_width {
        all_starts.push(LightRay {
            direction: Direction::DOWN,
            x_pos: with,
            y_pos: 0,
        });
        all_starts.push(LightRay {
            direction: Direction::LEFT,
            x_pos: with,
            y_pos: max_height - 1,
        });
    }

    let mut max_energized = 0;
    let mut best_start = LightRay {
        direction: Direction::LEFT,
        x_pos: 0,
        y_pos: 0,
    };

    for start_point in all_starts {
        let energy = calc_illumination(&mirrors, vec![start_point], true);
        if energy > max_energized {
            max_energized = energy;
            best_start = start_point;
        }
    }

    let energy = calc_illumination(&mirrors, vec![best_start], false);

    println!(
        "Part 1: {} Part 2 {} Took {:?} ",
        energized,
        energy,
        start.elapsed()
    )
}

struct MirWalk {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn calc_illumination(mirrors: &Vec<Vec<char>>, start: Vec<LightRay>, silent: bool) -> usize {
    let mut scores: Vec<Vec<MirWalk>> = mirrors
        .iter()
        .map(|f| {
            f.iter()
                .map(|_f| MirWalk {
                    up: false,
                    down: false,
                    left: false,
                    right: false,
                })
                .collect()
        })
        .collect();
    if !silent {
        print_board(&mirrors, &scores);
    }

    let mut run_remain: usize = mirrors.len() * mirrors.last().unwrap().len();

    let mut light_rays: Vec<LightRay> = start;

    if !silent {
        println!("Starting wit ray: {:?}", light_rays);
    }

    while light_rays.len() > 0 && run_remain > 0 {
        run_remain = run_remain - 1;
        if !silent {
            println!("Rays: {:?} time: {}", light_rays.len(), run_remain);
        }
        let mut nex_rays: Vec<LightRay> = vec![];
        for light_ray in light_rays {
            let mut rays = ray_move(light_ray, &mirrors, &mut scores);
            nex_rays.append(&mut rays);
        }
        light_rays = nex_rays;
    }
    if !silent {
        print_board(&mirrors, &scores);
        print_board_scores(mirrors, &scores);
    }
    let energized = get_board_scores(&mirrors, &scores);

    energized
}

fn ray_move(
    ray: LightRay,
    mirrors: &Vec<Vec<char>>,
    scores: &mut Vec<Vec<MirWalk>>,
) -> Vec<LightRay> {
    // println!("Ray: {:?} char at: {}", ray, mirrors[ray.y_pos][ray.x_pos]);

    let mirror_type = mirrors[ray.y_pos][ray.x_pos];

    let score = &mut scores[ray.y_pos][ray.x_pos];

    match ray.direction {
        Direction::DOWN => {
            if score.down {
                return vec![];
            } else {
                score.down = true;
            }
        }
        Direction::LEFT => {
            if score.left {
                return vec![];
            } else {
                score.left = true;
            }
        }
        Direction::RIGHT => {
            if score.right {
                return vec![];
            } else {
                score.right = true;
            }
        }
        Direction::UP => {
            if score.up {
                return vec![];
            } else {
                score.up = true;
            }
        }
    };

    let new_rays: Vec<LightRay> = match mirror_type {
        '|' => match ray.direction {
            Direction::DOWN => {
                vec![ray]
            }
            Direction::LEFT => {
                vec![
                    LightRay {
                        direction: Direction::UP,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                    LightRay {
                        direction: Direction::DOWN,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                ]
            }
            Direction::RIGHT => {
                vec![
                    LightRay {
                        direction: Direction::UP,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                    LightRay {
                        direction: Direction::DOWN,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                ]
            }
            Direction::UP => {
                vec![ray]
            }
        },
        '-' => match ray.direction {
            Direction::DOWN => {
                vec![
                    LightRay {
                        direction: Direction::LEFT,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                    LightRay {
                        direction: Direction::RIGHT,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                ]
            }
            Direction::LEFT => {
                vec![ray]
            }
            Direction::RIGHT => {
                vec![ray]
            }
            Direction::UP => {
                vec![
                    LightRay {
                        direction: Direction::LEFT,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                    LightRay {
                        direction: Direction::RIGHT,
                        x_pos: ray.x_pos,
                        y_pos: ray.y_pos,
                    },
                ]
            }
        },
        '\\' => match ray.direction {
            Direction::DOWN => {
                vec![LightRay {
                    direction: Direction::RIGHT,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::LEFT => {
                vec![LightRay {
                    direction: Direction::UP,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::RIGHT => {
                vec![LightRay {
                    direction: Direction::DOWN,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::UP => {
                vec![LightRay {
                    direction: Direction::LEFT,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
        },
        '/' => match ray.direction {
            Direction::DOWN => {
                vec![LightRay {
                    direction: Direction::LEFT,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::LEFT => {
                vec![LightRay {
                    direction: Direction::DOWN,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::RIGHT => {
                vec![LightRay {
                    direction: Direction::UP,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
            Direction::UP => {
                vec![LightRay {
                    direction: Direction::RIGHT,
                    x_pos: ray.x_pos,
                    y_pos: ray.y_pos,
                }]
            }
        },
        // '.' => {}
        _ => {
            vec![ray]
        }
    };

    let max_height: usize = mirrors.len() - 1;
    let max_width: usize = mirrors.last().unwrap().len() - 1;

    let next_ray: Vec<LightRay> = new_rays
        .iter()
        .filter_map(|ray_ray| match ray_ray.direction {
            Direction::DOWN => {
                if ray_ray.y_pos < max_height {
                    Some(LightRay {
                        direction: ray_ray.direction,
                        x_pos: ray_ray.x_pos,
                        y_pos: ray_ray.y_pos + 1,
                    })
                } else {
                    None
                }
            }
            Direction::LEFT => {
                if ray_ray.x_pos > 0 {
                    Some(LightRay {
                        direction: ray_ray.direction,
                        x_pos: ray_ray.x_pos - 1,
                        y_pos: ray_ray.y_pos,
                    })
                } else {
                    None
                }
            }
            Direction::RIGHT => {
                if ray_ray.x_pos < max_width {
                    Some(LightRay {
                        direction: ray_ray.direction,
                        x_pos: ray_ray.x_pos + 1,
                        y_pos: ray_ray.y_pos,
                    })
                } else {
                    None
                }
            }
            Direction::UP => {
                if ray_ray.y_pos > 0 {
                    Some(LightRay {
                        direction: ray_ray.direction,
                        x_pos: ray_ray.x_pos,
                        y_pos: ray_ray.y_pos - 1,
                    })
                } else {
                    None
                }
            }
        })
        .collect();

    next_ray

    // vec![]
}

fn print_board(mirrors: &Vec<Vec<char>>, scores: &Vec<Vec<MirWalk>>) -> usize {
    let mut score_total = 0;
    for mirror_line in mirrors.iter().enumerate() {
        print!("    ");
        for mirrors in mirror_line.1.iter().enumerate() {
            let score = &scores[mirror_line.0][mirrors.0];
            if score.down || score.left || score.right || score.up {
                print!("{}", mirrors.1.to_string().yellow());
                score_total = score_total + 1;
            } else {
                print!("{}", mirrors.1);
            }
        }
        println!();
    }
    println!();
    score_total
}

fn print_board_scores(mirrors: &Vec<Vec<char>>, scores: &Vec<Vec<MirWalk>>) -> usize {
    let mut score_total = 0;
    for mirror_line in mirrors.iter().enumerate() {
        print!("    ");
        for mirrors in mirror_line.1.iter().enumerate() {
            let score = &scores[mirror_line.0][mirrors.0];
            if score.down || score.left || score.right || score.up {
                print!("#");
                score_total = score_total + 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    score_total
}

fn get_board_scores(mirrors: &Vec<Vec<char>>, scores: &Vec<Vec<MirWalk>>) -> usize {
    let mut score_total = 0;
    for mirror_line in mirrors.iter().enumerate() {
        for mirrors in mirror_line.1.iter().enumerate() {
            let score = &scores[mirror_line.0][mirrors.0];
            if score.down || score.left || score.right || score.up {
                score_total = score_total + 1;
            }
        }
    }
    score_total
}
