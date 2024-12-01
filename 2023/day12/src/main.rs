use itertools::Itertools;
use std::fs;
use std::time::Instant;

// Part 1: 7007 Part 2: 3476169006222
fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut part1 = 0;
    let part2 = 0;

    for line in input.lines() {
        let texts = line.split_once(" ");
        let springs = texts.unwrap().0;
        let parity_list: Vec<usize> = texts
            .unwrap()
            .1
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let pattern = (0..5).map(|_| springs).join("?");
        let counts = parity_list.repeat(5);

        let count = count_arrangements(&pattern, &counts);

        println!("Line {} Count {}", springs, count);

        part1 = part1 + count;
    }

    println!(
        "Part 1: {} Part 2: {} took {:?}",
        part1,
        part2,
        start.elapsed()
    );
}

// from https://github.com/akaritakai/AdventOfCode2023/blob/main/src/day12.rs

fn count_arrangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}
