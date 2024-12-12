use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

use colored::{Colorize, CustomColor};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

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

        let mut hasher = DefaultHasher::new();
        current_id.hash(&mut hasher);
        let num_color = hasher.finish() % (256 * 256 * 256);
        let color: CustomColor = CustomColor {
            r: (num_color % 256) as u8,
            g: ((num_color / 256) % 256) as u8,
            b: ((num_color / (256 * 256)) % 256) as u8,
        };

        println!(
            "{} {} {} with id {} {}",
            nx.0,
            nx.1,
            nx.2,
            current_id,
            nx.0.to_string().custom_color(color)
        );

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

    let part_1 = regions
        .values()
        .map(|f| calc_price(f.clone()))
        .sum::<usize>();

    println!("Part 1 {}", part_1);
    println!("Time {:?}", start.elapsed());
}

fn print_board(letters: Vec<Vec<(char, usize)>>) {
    for line in letters {
        for f in line {
            let mut hasher = DefaultHasher::new();
            f.1.hash(&mut hasher);
            "lel".hash(&mut hasher);
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

fn calc_price(region: (char, Vec<(usize, usize)>)) -> usize {
    let area = region.1.len();

    let circum: usize = region
        .1
        .iter()
        .map(|r| {
            let row = r.0 as isize;
            let col = r.1 as isize;
            let up = region
                .1
                .iter()
                .find(|f| f.0 as isize == row - 1 && f.1 as isize == col);
            let down = region
                .1
                .iter()
                .find(|f| f.0 as isize == row + 1 && f.1 as isize == col);
            let left = region
                .1
                .iter()
                .find(|f| f.0 as isize == row && f.1 as isize == col - 1);
            let right = region
                .1
                .iter()
                .find(|f| f.0 as isize == row && f.1 as isize == col + 1);

            let sum = up.is_none() as usize
                + down.is_none() as usize
                + left.is_none() as usize
                + right.is_none() as usize;

            sum
        })
        .sum();

    println!(
        "Region {} Area {} Circum {} Price {}",
        region.0,
        area,
        circum,
        circum * area
    );

    circum * area
}

fn get_edges(squares: Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut edges: HashSet<(usize, usize)> = HashSet::new();

    edges
}
