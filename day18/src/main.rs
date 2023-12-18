use hex_color::HexColor;
use std::fs;

use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hole {
    x: i32,
    y: i32,
    color: HexColor,
    pipe: char,
}

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut lagoon: Vec<Hole> = vec![];

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    let mut last_hole: Hole = Hole {
        x: 0,
        y: 0,
        color: HexColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
        pipe: 'D',
    };

    let mut last_type = 'D';

    for line in input_lines {
        let line_parts: Vec<&str> = line.split(" ").collect();
        if line_parts.len() == 3 {
            let dir = line_parts[0].chars().nth(0).unwrap();
            let walk_distance: usize = line_parts[1].parse().unwrap();
            let trench_color: HexColor =
                HexColor::parse_rgb(&line_parts[2].replace("(", "").replace(")", "")).unwrap();
            println!(
                "Line dir {} walks {} color {:?}",
                dir, walk_distance, trench_color
            );
            for _index in 0..walk_distance {
                let hole: Hole = match dir {
                    'U' => Hole {
                        x: last_hole.x,
                        y: last_hole.y - 1,
                        color: trench_color,
                        pipe: '.',
                    },
                    'D' => Hole {
                        x: last_hole.x,
                        y: last_hole.y + 1,
                        color: trench_color,
                        pipe: '.',
                    },
                    'L' => Hole {
                        x: last_hole.x - 1,
                        y: last_hole.y,
                        color: trench_color,
                        pipe: '.',
                    },
                    'R' => Hole {
                        x: last_hole.x + 1,
                        y: last_hole.y,
                        color: trench_color,
                        pipe: '.',
                    },
                    _ => Hole {
                        x: last_hole.x,
                        y: last_hole.y,
                        color: trench_color,
                        pipe: '.',
                    },
                };
                lagoon
                    .last_mut()
                    .unwrap_or(&mut Hole {
                        x: 0,
                        y: 0,
                        color: HexColor {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 0,
                        },
                        pipe: ' ',
                    })
                    .pipe = get_next_pipe(last_type, dir);
                last_hole = hole;
                last_type = dir;
                lagoon.push(hole);
                if hole.x > max_x {
                    max_x = hole.x
                }
                if hole.x < min_x {
                    min_x = hole.x
                }
                if hole.y > max_y {
                    max_y = hole.y
                }
                if hole.y < min_y {
                    min_y = hole.y
                }
            }
        }
    }

    lagoon
        .last_mut()
        .unwrap_or(&mut Hole {
            x: 0,
            y: 0,
            color: HexColor {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            pipe: ' ',
        })
        .pipe = get_next_pipe(last_type, 'R');

    lagoon.sort();

    println!(
        "min x {} max x {} min y {} max y {} Holes: {:?} Area: {}",
        min_x,
        max_x,
        min_y,
        max_y,
        lagoon.len(),
        shoelace_formula(&lagoon)
    );

    let lagoon_bits = get_lagoon_vec(lagoon, min_x, min_y, max_x, max_y);

    print_lagoon_bits(&lagoon_bits);

    println!();

    let count = calc_inside(&lagoon_bits);
    // let count = get_inside_lagoon_gpt(&lagoon_bits);

    print_lagoon_bits(&count.1);

    println!("Solution: {} Took {:?}", count.0, start.elapsed())
}

fn get_next_pipe(before: char, now: char) -> char {
    match (before, now) {
        ('U', 'D') => '│',
        ('D', 'U') => '│',
        ('U', 'U') => '│',
        ('D', 'D') => '│',
        ('L', 'R') => '─',
        ('R', 'L') => '─',
        ('R', 'R') => '─',
        ('L', 'L') => '─',
        ('U', 'L') => '┐',
        ('L', 'U') => '└',
        ('D', 'R') => '└',
        ('R', 'D') => '┐',
        ('U', 'R') => '┌',
        ('R', 'U') => '┘',
        ('D', 'L') => '┘',
        ('L', 'D') => '┌',
        _ => {
            println!("Error: {} {}", before, now);
            '.'
        } // Default case
    }
}

fn get_lagoon_vec(
    lagoon: Vec<Hole>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> Vec<Vec<char>> {
    let mut bit_lagoon: Vec<Vec<char>> = ((min_y)..(max_y + 1))
        .map(|_f| ((min_x)..(max_x + 1)).map(|_f| '.').collect())
        .collect();

    for hole in lagoon {
        let x_i: usize = (hole.x - min_x).try_into().unwrap();
        let y_i: usize = (hole.y - min_y).try_into().unwrap();
        bit_lagoon[y_i][x_i] = hole.pipe;
    }

    bit_lagoon
}

fn calc_inside(lagoon: &Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    let mut in_count = 0;

    let mut algos = lagoon.clone();

    for pipe_line in lagoon.iter().enumerate() {
        let mut inside = false;
        let mut last_pipe = ' ';

        for pipe in pipe_line.1.iter().enumerate() {
            let pipe_type = pipe.1;

            if inside || pipe_type != &'.' {
                in_count += 1;
            }

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
                '.' => {
                    if inside {
                        algos[pipe_line.0][pipe.0] = '#'
                    }
                }
                _ => {}
            }
        }
    }

    (in_count, algos)
}

fn print_lagoon_bits(lagoon: &Vec<Vec<char>>) {
    for bit_line in lagoon {
        for bit in bit_line {
            print!("{}", bit)
        }
        println!()
    }
}

fn shoelace_formula(points: &[Hole]) -> f64 {
    let n = points.len();
    let mut area = 0.0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].x * points[j].y - points[j].x * points[i].y) as f64;
    }

    area = 0.5 * area.abs();
    area
}
