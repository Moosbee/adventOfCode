use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fs,
    time::Instant,
};

fn main() {
    let start = Instant::now();

    println!("I admit defeat, copied from https://github.com/akaritakai/AdventOfCode2023/blob/main/src/day21.rs");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let grid = Grid::parse(&input);
    let part_1 = grid.calculate_reachable_plots(64).to_string();
    let grid = Grid::parse(&input);
    let part_2 = grid.calculate_reachable_plots(26501365).to_string();

    println!(
        "Solution:1: {} 2: {} Took {:?}",
        part_1,
        part_2,
        start.elapsed()
    )
}

struct Grid {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut start = (0, 0);
        for (row, line) in grid.iter().enumerate() {
            for (col, &c) in line.iter().enumerate() {
                if c == 'S' {
                    start = (row, col);
                }
            }
        }
        Grid { grid, start }
    }

    fn calculate_reachable_plots(&self, steps: usize) -> usize {
        let mut distances: HashMap<(isize, isize, usize, usize), u64> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0, self.start.0, self.start.1), 0));
        distances.insert((0, 0, self.start.0, self.start.1), 0);
        while let Some((coordinates, dist)) = queue.pop_front() {
            for successor in self.successors(coordinates) {
                if successor.0.abs() > 1 || successor.1.abs() > 1 {
                    continue;
                }
                if let Entry::Vacant(e) = distances.entry(successor) {
                    e.insert(dist + 1);
                    queue.push_back((successor, dist + 1));
                }
            }
        }

        let n = self.grid.len();
        let mut dp = vec![0; steps + 2 * n + 1];
        for i in (0..=steps).rev() {
            dp[i] = ((i % 2 == steps % 2) as usize) + 2 * dp[i + n] - dp[i + 2 * n];
        }
        let mut reachable = 0;
        for (coordinates, distance) in distances.iter() {
            if *distance <= steps as u64 {
                let n = self.grid.len() as isize;
                let (row_dist, col_dist) =
                    self.manhattan_distance(*coordinates, (0, 0, self.start.0, self.start.1));
                if (-n..n).contains(&row_dist) && (-n..n).contains(&col_dist) {
                    reachable += dp[*distance as usize];
                }
            }
        }
        reachable
    }

    fn successors(
        &self,
        coordinates: (isize, isize, usize, usize),
    ) -> Vec<(isize, isize, usize, usize)> {
        let (num_rows, num_cols) = (self.grid.len(), self.grid[0].len());
        let (trans_row, trans_col, row, col) = coordinates;
        let mut successors = Vec::new();
        // Check up
        if row > 0 && self.grid[row - 1][col] != '#' {
            successors.push((trans_row, trans_col, row - 1, col));
        } else if row == 0 {
            successors.push((trans_row - 1, trans_col, num_rows - 1, col));
        }
        // Check down
        if row < num_rows - 1 && self.grid[row + 1][col] != '#' {
            successors.push((trans_row, trans_col, row + 1, col));
        } else if row == num_rows - 1 {
            successors.push((trans_row + 1, trans_col, 0, col));
        }
        // Check left
        if col > 0 && self.grid[row][col - 1] != '#' {
            successors.push((trans_row, trans_col, row, col - 1));
        } else if col == 0 {
            successors.push((trans_row, trans_col - 1, row, num_cols - 1));
        }
        // Check right
        if col < num_cols - 1 && self.grid[row][col + 1] != '#' {
            successors.push((trans_row, trans_col, row, col + 1));
        } else if col == num_cols - 1 {
            successors.push((trans_row, trans_col + 1, row, 0));
        }
        successors
    }

    fn manhattan_distance(
        &self,
        a: (isize, isize, usize, usize),
        b: (isize, isize, usize, usize),
    ) -> (isize, isize) {
        let n = self.grid.len() as isize;
        let (a_row, a_col) = (a.2 as isize + a.0 * n, a.3 as isize + a.1 * n);
        let (b_row, b_col) = (b.2 as isize + b.0 * n, b.3 as isize + b.1 * n);
        (a_row - b_row, a_col - b_col)
    }
}
