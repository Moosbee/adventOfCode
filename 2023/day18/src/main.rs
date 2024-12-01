use std::fs;

use std::time::Instant;

struct Instruction {
    direction: char,
    distance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Hole {
    x: f64,
    y: f64,
}

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut lagoon: Vec<Hole> = vec![];

    let mut min_x = 0.0;
    let mut min_y = 0.0;
    let mut max_x = 0.0;
    let mut max_y = 0.0;

    let mut last_hole: Hole = Hole { x: 0.5, y: 0.5 };

    let mut last_dir = 'R';
    let mut extra_mass = 0.0;

    for line in input_lines {
        let inst = gen_instruction(line).unwrap();

        let hole: Hole = match inst.direction {
            'U' => Hole {
                x: last_hole.x,
                y: last_hole.y - inst.distance,
            },
            'D' => Hole {
                x: last_hole.x,
                y: last_hole.y + inst.distance,
            },
            'L' => Hole {
                x: last_hole.x - inst.distance,
                y: last_hole.y,
            },
            'R' => Hole {
                x: last_hole.x + inst.distance,
                y: last_hole.y,
            },
            _ => Hole {
                x: last_hole.x,
                y: last_hole.y,
            },
        };

        extra_mass = extra_mass + calc_extra_mass(last_dir, inst.direction, inst.distance);

        last_hole = hole;
        last_dir = inst.direction;
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

    list_points(&lagoon, "R");

    println!(
        "min x {} max x {} min y {} max y {} Holes: {:?} Area: {} winding: {} extra mass: {}",
        min_x,
        max_x,
        min_y,
        max_y,
        lagoon.len(),
        shoelace_formula(&lagoon),
        calculate_winding_order(&lagoon),
        extra_mass
    );

    println!(
        "Solution: {} Took {:?}",
        shoelace_formula(&lagoon) + extra_mass+1.0,
        start.elapsed()
    )
}

// Expected 62
// Got      62

// Expected 47675
// Got      47675

// Expected 952408144115
// Got      952408144115

// Expected 122103860427465
// Got      122103860427465

fn calc_extra_mass(before: char, now: char, distance: f64) -> f64 {
    match (before, now) {
        ('U', 'D') => 0.5 * distance,
        ('D', 'U') => 0.5 * distance,
        ('U', 'U') => 0.5 * distance,
        ('D', 'D') => 0.5 * distance,
        ('L', 'R') => 0.5 * distance,
        ('R', 'L') => 0.5 * distance,
        ('R', 'R') => 0.5 * distance,
        ('L', 'L') => 0.5 * distance,

        // D (1.0, 0.0) Down  ↓
        // L (0.0, 1.0) Left  ←
        // R (0.0, 0.0) Right →
        // U (0.0, 0.0) UP    ↑
        ('U', 'L') => 0.5 * distance + 0.0,
        ('R', 'U') => 0.5 * distance + 0.0,
        ('R', 'D') => 0.5 * distance + 0.0,
        ('D', 'L') => 0.5 * distance + 0.0,
        ('L', 'U') => 0.5 * distance + 0.0,
        ('D', 'R') => 0.5 * distance + 0.0,
        ('U', 'R') => 0.5 * distance + 0.0,
        ('L', 'D') => 0.5 * distance + 0.0,
        _ => {
            println!("Error: {} {}", before, now);
            0.0
        } // Default case
    }
}

fn list_points(points: &[Hole], p_name: &str) {
    let mut poly_command = "\\operatorname{polygon}\\left(".to_string();
    for (index, point) in points.iter().enumerate() {
        println!(
            "{}_{{{}}}=\\left({},{}\\right)",
            p_name, index, point.x, -point.y
        );
        poly_command = poly_command + p_name + "_{" + &index.to_string() + "},\\ ";
    }
    poly_command = poly_command.trim_end_matches(",\\ ").to_owned() + "\\right)";
    println!("{}", poly_command);
}


fn shoelace_formula(points: &[Hole]) -> f64 {
    let n = points.len();
    let mut area: f64 = 0.0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].x * points[j].y - points[j].x * points[i].y) as f64;
    }

    area = 0.5 * area.abs();
    area
}

fn gen_instruction(text: &str) -> Option<Instruction> {
    let line_parts: Vec<&str> = text.split(" ").collect();
    if line_parts.len() == 3 {
        // println!(
        //     "Line dir {} walks {} color {:?}",
        //     dir, walk_distance, trench_color
        // );
        if false {
            let dir = line_parts[0].chars().nth(0).unwrap();
            let walk_distance = line_parts[1].parse().unwrap();
            Some(Instruction {
                direction: dir,
                distance: walk_distance,
            })
        } else {
            let trench_color = line_parts[2]
                .replace("(", "")
                .replace(")", "")
                .replace("#", "");

            let str_size = trench_color.len();
            let last_char = &trench_color[(str_size - 1)..];
            let distance = i64::from_str_radix(&trench_color[..(str_size - 1)], 16);

            println!("Dist: {:?} dir {}", distance, last_char);

            Some(Instruction {
                direction: match last_char {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    _ => {
                        // Handle other cases if needed
                        // This block will be executed if the number doesn't match any of the specified cases
                        // You can return an error or handle it based on your requirements
                        panic!("Invalid number");
                    }
                },
                distance: distance.unwrap() as f64,
            })
        }
    } else {
        None
    }
}

// If the signed area is positive, the winding order is counterclockwise.
// If the signed area is negative, the winding order is clockwise.
fn calculate_winding_order(vertices: &Vec<Hole>) -> f64 {
    let n = vertices.len();
    let mut signed_area: f64 = 0.0;

    for i in 0..n {
        let point_1 = vertices[i];
        let point_2 = vertices[(i + 1) % n]; // Loop back to the first vertex for the last edge
        signed_area += (point_2.x - point_1.x) * (point_2.y + point_1.y);
    }
    signed_area
}
