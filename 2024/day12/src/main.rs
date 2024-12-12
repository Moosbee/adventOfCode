use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

use colored::{Colorize, CustomColor};

const HASH_NUM: &str = "kek";

fn main() {
    let input =
        fs::read_to_string("./test_input_1.txt").expect("Should have been able to read the file");
    // let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let mut letters: Vec<Vec<(char, usize)>> = input_lines
        .map(|line| line.trim().chars().map(|f| (f, 0)).collect())
        .collect::<Vec<Vec<(char, usize)>>>();

    let mut regions: HashMap<usize, (char, Vec<(usize, usize)>)> = HashMap::new();

    let mut to_visit: VecDeque<(char, usize, usize)> = VecDeque::new();

    let firs = letters[0][0];

    let mut next: Vec<(char, usize, usize)> = vec![(firs.0, 0, 0)];

    let mut current_id: usize = 1;

    while let Some(nx) = next.pop() {
        let firs = letters[nx.1][nx.2];
        if firs.1 != 0 {
            continue;
        }
        to_visit.push_back((firs.0, nx.1, nx.2));
        current_id += 2;

        // let mut hasher = DefaultHasher::new();
        // current_id.hash(&mut hasher);
        // let num_color = hasher.finish() % (256 * 256 * 256);
        // let color: CustomColor = CustomColor {
        //     r: (num_color % 256) as u8,
        //     g: ((num_color / 256) % 256) as u8,
        //     b: ((num_color / (256 * 256)) % 256) as u8,
        // };

        // println!(
        //     "{} {} {} with id {} {}",
        //     nx.0,
        //     nx.1,
        //     nx.2,
        //     current_id,
        //     nx.0.to_string().custom_color(color)
        // );

        while let Some(visit) = to_visit.pop_front() {
            let current = letters
                .get_mut(visit.1)
                .map(|f| f.get_mut(visit.2))
                .flatten()
                .unwrap();

            if current.1 != 0 {
                continue;
            }

            current.1 = current_id;
            let mut rer = regions.remove(&current_id).unwrap_or((visit.0, Vec::new()));

            rer.1.push((visit.1, visit.2));

            regions.insert(current_id.clone(), rer);

            let mut push = |dir: (isize, isize)| {
                let row = visit.1 as isize + dir.0;
                let col = visit.2 as isize + dir.1;

                if row >= 0
                    && row < letters.len() as isize
                    && col >= 0
                    && col < letters[0].len() as isize
                {
                    let (ch, id) = letters[row as usize][col as usize];

                    if ch == visit.0 && id == 0 {
                        to_visit.push_back((ch, row as usize, col as usize));
                    } else if id == 0 {
                        next.push((ch, row as usize, col as usize)); // = Some((ch, row as usize, col as usize));
                    }
                }
            };

            push((1, 0));
            push((0, 1));
            push((-1, 0));
            push((0, -1));
        }
        // next = None;
    }

    print_board(letters);

    let erg = regions
        .iter()
        .map(|f| calc_price((f.0.clone(), f.1 .0.clone(), f.1 .1.clone())))
        .fold((0, 0), |acc, f| (acc.0 + f.0, acc.1 + f.1));

    println!("Part 1 {}", erg.0);
    println!("Part 2 {}", erg.1);
    println!("Time {:?}", start.elapsed());
}

fn print_board(letters: Vec<Vec<(char, usize)>>) {
    for line in letters {
        for f in line {
            let mut hasher = DefaultHasher::new();
            f.1.hash(&mut hasher);
            HASH_NUM.hash(&mut hasher);
            let num_color = hasher.finish() % (256 * 256 * 256);
            let color: CustomColor = CustomColor {
                r: (num_color % 256) as u8,
                g: ((num_color / 256) % 256) as u8,
                b: ((num_color / (256 * 256)) % 256) as u8,
            };
            print!("{}", f.0.to_string().custom_color(color));
        }
        println!();
    }
}

fn calc_price(region: (usize, char, Vec<(usize, usize)>)) -> (usize, usize) {
    let area = region.2.len();

    let circum_1: usize = region
        .2
        .iter()
        .map(|r| {
            let row = r.0 as isize;
            let col = r.1 as isize;
            let up = region
                .2
                .iter()
                .find(|f| f.0 as isize == row - 1 && f.1 as isize == col);
            let down = region
                .2
                .iter()
                .find(|f| f.0 as isize == row + 1 && f.1 as isize == col);
            let left = region
                .2
                .iter()
                .find(|f| f.0 as isize == row && f.1 as isize == col - 1);
            let right = region
                .2
                .iter()
                .find(|f| f.0 as isize == row && f.1 as isize == col + 1);

            let sum = up.is_none() as usize
                + down.is_none() as usize
                + left.is_none() as usize
                + right.is_none() as usize;

            sum
        })
        .sum();

    let points = get_points(region.2);
    let sides: usize = calc_sides(points);

    let mut hasher = DefaultHasher::new();
    region.0.hash(&mut hasher);
    HASH_NUM.hash(&mut hasher);
    let num_color = hasher.finish() % (256 * 256 * 256);
    let color: CustomColor = CustomColor {
        r: (num_color % 256) as u8,
        g: ((num_color / 256) % 256) as u8,
        b: ((num_color / (256 * 256)) % 256) as u8,
    };

    // println!("Circum 1 {} Circum 2 {}", circum_1, circum_2);
    println!(
        "Region {} Area {} Circum {} Sides {} Price {} APrice {} id {}",
        region.1.to_string().custom_color(color),
        area,
        circum_1,
        sides,
        circum_1 * area,
        sides * area,
        region.0
    );

    (circum_1 * area, sides * area)
}

fn get_points(squares: Vec<(usize, usize)>) -> HashSet<((usize, usize), (usize, usize))> {
    let mut mini_sides: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    for sq in squares.iter() {
        let row = sq.0 as isize;
        let col = sq.1 as isize;
        let up = squares
            .iter()
            .find(|f| f.0 as isize == row - 1 && f.1 as isize == col);
        let right = squares
            .iter()
            .find(|f| f.0 as isize == row && f.1 as isize == col + 1);
        let down = squares
            .iter()
            .find(|f| f.0 as isize == row + 1 && f.1 as isize == col);
        let left = squares
            .iter()
            .find(|f| f.0 as isize == row && f.1 as isize == col - 1);
        if up.is_none() {
            mini_sides.insert((
                (row as usize, col as usize),
                (row as usize, (col + 1) as usize),
            ));
        }
        if down.is_none() {
            mini_sides.insert((
                ((row + 1) as usize, col as usize),
                ((row + 1) as usize, (col + 1) as usize),
            ));
        }
        if left.is_none() {
            mini_sides.insert((
                (row as usize, col as usize),
                ((row + 1) as usize, col as usize),
            ));
        }
        if right.is_none() {
            mini_sides.insert((
                (row as usize, (col + 1) as usize),
                ((row + 1) as usize, (col + 1) as usize),
            ));
        }
    }

    mini_sides
}

fn calc_sides(mut sides: HashSet<((usize, usize), (usize, usize))>) -> usize {
    let all_points: HashSet<(usize, usize)> = sides
        .iter()
        .flat_map(|&((x1, y1), (x2, y2))| [(x1, y1), (x2, y2)])
        .collect();

    for current in all_points.iter() {
        let row = current.0 as isize;
        let col = current.1 as isize;

        let up = sides
            .iter()
            .find(|f| {
                (f.0 .0 as isize == row - 1
                    && f.0 .1 as isize == col
                    && f.1 .0 as isize == row
                    && f.1 .1 as isize == col)
                    || (f.1 .0 as isize == row - 1
                        && f.1 .1 as isize == col
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col)
            })
            .is_some();
        let down = sides
            .iter()
            .find(|f| {
                (f.0 .0 as isize == row + 1
                    && f.0 .1 as isize == col
                    && f.1 .0 as isize == row
                    && f.1 .1 as isize == col)
                    || (f.1 .0 as isize == row + 1
                        && f.1 .1 as isize == col
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col)
            })
            .is_some();
        let right = sides
            .iter()
            .find(|f| {
                (f.0 .0 as isize == row
                    && f.0 .1 as isize == col + 1
                    && f.1 .0 as isize == row
                    && f.1 .1 as isize == col)
                    || (f.1 .0 as isize == row
                        && f.1 .1 as isize == col + 1
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col)
            })
            .is_some();
        let left = sides
            .iter()
            .find(|f| {
                (f.0 .0 as isize == row
                    && f.0 .1 as isize == col - 1
                    && f.1 .0 as isize == row
                    && f.1 .1 as isize == col)
                    || (f.1 .0 as isize == row
                        && f.1 .1 as isize == col - 1
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col)
            })
            .is_some();

        println!("jinikln {:?}, {} {} {} {}", current, up, down, left, right);

        if (up && down) && !(left || right) {
            let up_left = sides
                .iter()
                .find(|f| {
                    f.1 .0 as isize == row - 1
                        && f.1 .1 as isize == col
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col
                })
                .cloned();
            let up_right = sides
                .iter()
                .find(|f| {
                    f.0 .0 as isize == row - 1
                        && f.0 .1 as isize == col
                        && f.1 .0 as isize == row
                        && f.1 .1 as isize == col
                })
                .cloned();
            let down_left = sides
                .iter()
                .find(|f| {
                    f.1 .0 as isize == row + 1
                        && f.1 .1 as isize == col
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col
                })
                .cloned();

            let down_right = sides
                .iter()
                .find(|f| {
                    f.0 .0 as isize == row + 1
                        && f.0 .1 as isize == col
                        && f.1 .0 as isize == row
                        && f.1 .1 as isize == col
                })
                .cloned();

            let down_side = down_left.unwrap_or_else(|| down_right.unwrap());
            let down = down_left
                .map(|f| f.1)
                .unwrap_or_else(|| down_right.unwrap().0);
            let up_side = up_left.unwrap_or_else(|| up_right.unwrap());
            let up = up_left.map(|f| f.1).unwrap_or_else(|| up_right.unwrap().0);

            let new_side = (up, down);
            sides.remove(&up_side);
            sides.remove(&down_side);
            sides.insert(new_side);
        } else if !(up || down) && (left && right) {
            let left_left = sides
                .iter()
                .find(|f| {
                    f.1 .0 as isize == row
                        && f.1 .1 as isize == col - 1
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col
                })
                .cloned();
            let left_right = sides
                .iter()
                .find(|f| {
                    f.0 .0 as isize == row
                        && f.0 .1 as isize == col - 1
                        && f.1 .0 as isize == row
                        && f.1 .1 as isize == col
                })
                .cloned();
            let right_left = sides
                .iter()
                .find(|f| {
                    f.1 .0 as isize == row
                        && f.1 .1 as isize == col + 1
                        && f.0 .0 as isize == row
                        && f.0 .1 as isize == col
                })
                .cloned();

            let right_right = sides
                .iter()
                .find(|f| {
                    f.0 .0 as isize == row
                        && f.0 .1 as isize == col + 1
                        && f.1 .0 as isize == row
                        && f.1 .1 as isize == col
                })
                .cloned();

            let right_side = right_left.unwrap_or_else(|| right_right.unwrap());
            let right = right_left
                .map(|f| f.1)
                .unwrap_or_else(|| right_right.unwrap().0);
            let left_side = left_left.unwrap_or_else(|| left_right.unwrap());
            let left = left_left
                .map(|f| f.1)
                .unwrap_or_else(|| left_right.unwrap().0);

            let new_side = (left, right);
            sides.remove(&left_side);
            sides.remove(&right_side);
            sides.insert(new_side);
        } else {
        }
    }

    println!("Sides {:?}", sides);
    sides.len()
}
