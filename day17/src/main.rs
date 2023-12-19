use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let heat: Vec<Vec<usize>> = input_lines
        .map(|line| {
            line.chars()
                .filter_map(|f| f.to_digit(10).and_then(|l| Some(l as usize)))
                .collect()
        })
        .collect();

      println!("Copy paste from https://github.com/akaritakai/AdventOfCode2023/blob/main/src/day17.rs");

    let part_1 = least_heat_loss::<1, 3>(&heat);
    let part_2 = least_heat_loss::<4, 10>(&heat);
    println!("Solution 1: {} Solution 2: {} Took {:?}", part_1,part_2, start.elapsed())
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct LavaFlow {
    loc: (isize, isize),
    dir: (isize, isize),
    count: usize,
}

fn least_heat_loss<const MIN_STEPS: usize, const MAX_STEPS: usize>(grid: &[Vec<usize>]) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let start = (0, 0);
    let end = (rows as isize - 1, cols as isize - 1);

    let mut distance = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push((
        Reverse(0),
        LavaFlow {
            loc: start,
            dir: (0, 1),
            count: 0,
        },
    ));
    queue.push((
        Reverse(0),
        LavaFlow {
            loc: start,
            dir: (1, 0),
            count: 0,
        },
    ));

    while let Some((Reverse(cost), flow)) = queue.pop() {
        if flow.loc == end && flow.count >= MIN_STEPS {
            return cost;
        }
        for new_flow in neighbors::<MIN_STEPS, MAX_STEPS>(rows, cols, flow) {
            let new_cost = cost + grid[new_flow.loc.0 as usize][new_flow.loc.1 as usize];
            if new_cost < distance.get(&new_flow).copied().unwrap_or(usize::MAX) {
                distance.insert(new_flow, new_cost);
                queue.push((Reverse(new_cost), new_flow));
            }
        }
    }

    unreachable!()
}

fn neighbors<const MIN_STEPS: usize, const MAX_STEPS: usize>(
    rows: usize,
    cols: usize,
    flow: LavaFlow,
) -> impl Iterator<Item = LavaFlow> {
    let (row, col, drow, dcol, count) =
        (flow.loc.0, flow.loc.1, flow.dir.0, flow.dir.1, flow.count);
    let in_bounds = move |r, c| r >= 0 && r < rows as isize && c >= 0 && c < cols as isize;
    [(drow, dcol), (dcol, -drow), (-dcol, drow)]
        .into_iter()
        .filter_map(move |(new_drow, new_dcol)| {
            let is_turn = new_drow != drow || new_dcol != dcol;
            let (new_row, new_col) = (row + new_drow, col + new_dcol);
            if (is_turn && count < MIN_STEPS)
                || (!is_turn && count == MAX_STEPS)
                || !in_bounds(new_row, new_col)
            {
                None
            } else {
                Some(LavaFlow {
                    loc: (new_row, new_col),
                    dir: (new_drow, new_dcol),
                    count: if is_turn { 1 } else { count + 1 },
                })
            }
        })
}
