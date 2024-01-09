use std::fs;

use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BrickI {
    id: char,
    start: PointI,
    end: PointI,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PointI {
    x: i32,
    y: i32,
    z: i32,
}

fn to_point(text: &str) -> Option<PointI> {
    let mut values = text.split(',');

    Some(PointI {
        x: values.next()?.parse().ok()?,
        y: values.next()?.parse().ok()?,
        z: values.next()?.parse().ok()?,
    })

    // None
}

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    //expect 434

    // 574
    // 575
    // 492

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut bricks: Vec<BrickI> = vec![];
    let mut start_char: u8 = b'A';

    for line in input_lines {
        let (point_a, point_b) = line.split_once('~').unwrap();
        bricks.push(BrickI {
            id: start_char.into(),
            start: to_point(point_a).unwrap(),
            end: to_point(point_b).unwrap(),
        });

        // if start_char > b'Z' {
        //     start_char = b'A';
        // } else {
        //     start_char = start_char + 1;
        // }
    }

    print_bricks(&bricks);
    println!();
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    // Sort the vector based on the z field of the Point struct
    // bricks.sort();

    // println!("{:?}", bricks);
    print_bricks(&bricks);
    println!();

    let mut has_diff = true;
    let mut nombs = vec![];

    while has_diff && nombs.len() < 20 {
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
        (bricks, has_diff) = drop_bricks(bricks);
        let save_kills = save_disintegration(&bricks);
        nombs.push(save_kills);
        print_bricks(&bricks);
        println!();
    }

    // bricks.reverse();
    // bricks = drop_bricks(bricks);
    // bricks.reverse();
    // bricks = drop_bricks(bricks);
    // println!("{:?}", bricks);
    // println!();

    print_bricks(&bricks);
    println!();

    let save_kills = save_disintegration(&bricks);

    println!(
        "Solution: {} {:?} Took {:?}",
        save_kills,
        nombs,
        start.elapsed()
    );

    // let mut space = parse_input(&input);
    // space.fall_bricks_down();

    // for brk in &space.bricks {
    //     if !bricks.contains(&BrickI {
    //         id: 'A',
    //         start: PointI {
    //             x: brk.start.x as i32,
    //             y: brk.start.y as i32,
    //             z: brk.start.z as i32,
    //         },
    //         end: PointI {
    //             x: brk.end.x as i32,
    //             y: brk.end.y as i32,
    //             z: brk.end.z as i32,
    //         },
    //     }) {
    //         println!("{:?} not in list", brk);
    //     }
    // }

    // for brk in &bricks {
    //     if !space.bricks.contains(&Brick::new(
    //         Point {
    //             x: brk.start.x as isize,
    //             y: brk.start.y as isize,
    //             z: brk.start.z as isize,
    //         },
    //         Point {
    //             x: brk.end.x as isize,
    //             y: brk.end.y as isize,
    //             z: brk.end.z as isize,
    //         },
    //     )) {
    //         println!("{:?} not in better list", brk);
    //     }
    // }

    // println!("Solution: {} Took {:?}", part1(input), start.elapsed());
}

fn save_disintegration(bricks: &Vec<BrickI>) -> i32 {
    let mut count = 0;
    for brick in bricks {
        println!(
            "{:?} sits on {:?} holds {:?} {} would fall",
            brick,
            get_relative_bricks(PointI { x: 0, y: 0, z: -1 }, brick, bricks),
            get_relative_bricks(PointI { x: 0, y: 0, z: 1 }, brick, bricks),
            how_many_fall(brick, bricks)
        );
        if how_many_fall(brick, bricks) == 0 {
            count += 1;
        }
    }
    count
}

fn how_many_fall(brick: &BrickI, bricks: &Vec<BrickI>) -> i32 {
    let mut count = 0;
    let above_bricks: Vec<&BrickI> =
        get_relative_bricks(PointI { x: 0, y: 0, z: 1 }, brick, bricks);
    for up_brick in above_bricks {
        let standing_on_bricks =
            get_relative_bricks(PointI { x: 0, y: 0, z: -1 }, up_brick, bricks);
        if standing_on_bricks.len() == 1 {
            count = count + 1;
        }
    }

    count
}

fn get_relative_bricks<'a>(
    offset: PointI,
    brick: &'a BrickI,
    bricks: &'a Vec<BrickI>,
) -> Vec<&'a BrickI> {
    let mut above_bricks: Vec<&BrickI> = Vec::new();

    for pos in get_brick_tiles(brick) {
        if let Some(brick_in) = get_brick_at(
            PointI {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
                z: pos.z + offset.z,
            },
            bricks,
        ) {
            if brick_in != brick {
                above_bricks.push(brick_in);
            }
        }
    }

    above_bricks.sort();
    above_bricks.dedup();

    above_bricks
}

fn get_brick_at(point: PointI, bricks: &Vec<BrickI>) -> Option<&BrickI> {
    for brick in bricks {
        if {
            point.x >= brick.start.x
                && point.x <= brick.end.x
                && point.y >= brick.start.y
                && point.y <= brick.end.y
                && point.z >= brick.start.z
                && point.z <= brick.end.z
        } {
            return Some(brick);
        }
    }
    None
}

fn get_brick_tiles(brick: &BrickI) -> Vec<PointI> {
    let mut tiles = Vec::new();

    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            for z in brick.start.z..=brick.end.z {
                tiles.push(PointI { x, y, z });
            }
        }
    }

    tiles
}

fn drop_bricks(mut bricks: Vec<BrickI>) -> (Vec<BrickI>, bool) {
    let mut has_diff = false;
    for brick_index in 0..bricks.len() {
        let brick_copy = bricks.clone();
        let brick = brick_copy[brick_index];
        let low_z = brick.start.z.min(brick.end.z);
        let drop = drop_height(&brick, &brick_copy); // Pass mutable reference
        let diff = low_z - drop;
        if diff != 0 {
            has_diff = true;
        }
        bricks[brick_index].start.z = bricks[brick_index].start.z - diff;
        bricks[brick_index].end.z = bricks[brick_index].end.z - diff;
    }

    (bricks, has_diff)
}

fn drop_height(brick: &BrickI, bricks: &Vec<BrickI>) -> i32 {
    let mut drop = 0;
    // println!("\nP{:?}", brick);
    for brik in bricks {
        if intersect(brick, brik) && brik.start.z <= brick.start.z && brick != brik {
            // println!("B{:?}", brik);
            drop = brik.end.z + 1;
        }
    }

    drop
}

fn intersect(brick_a: &BrickI, brick_b: &BrickI) -> bool {
    // Check for x-axis overlap
    let x_overlap = brick_a.start.x <= brick_b.end.x && brick_a.end.x >= brick_b.start.x;

    // Check for y-axis overlap
    let y_overlap = brick_a.start.y <= brick_b.end.y && brick_a.end.y >= brick_b.start.y;

    // The bricks intersect if there is overlap in both dimensions
    x_overlap && y_overlap
}
