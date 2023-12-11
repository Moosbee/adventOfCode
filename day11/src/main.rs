use std::fs;

use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let expanded_sky = expand_sky(input_lines.map(|line| line.chars().collect()).collect());

    let galaxies: Vec<Galaxy> = expanded_sky
        .iter()
        .enumerate()
        .map(|f| {
            f.1.iter().enumerate().filter_map(move |g| {
                if g.1 == &'#' {
                    return Some(Galaxy { x: f.0, y: g.0 });
                }
                None
            })
        })
        .flatten()
        .collect();

    for exp in expanded_sky {
        for run in exp {
            print!("{}", run);
        }
        println!();
    }

    println!("Galaxies: {:?}", galaxies);

    let mut galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = vec![];

    for galaxy1 in &galaxies {
        for galaxy2 in &galaxies {
            let exists = galaxy1 == galaxy2
                || galaxy_pairs.iter().any(|f| {
                    (f.0 == galaxy1 && f.1 == galaxy2) || (f.1 == galaxy1 && f.0 == galaxy2)
                });

            if !exists {
                galaxy_pairs.push((galaxy1, galaxy2));
            }
        }
    }

    let mut sum = 0;

    for pair in galaxy_pairs {
        let distance = ((pair.0.x as i32) - (pair.1.x as i32)).abs()
            + ((pair.0.y as i32) - (pair.1.y as i32)).abs();
        sum = sum + distance;
        println!("Galaxy Pair: {:?} {}", pair, distance);
    }

    println!("Part 1: {} Part 2: {} took {:?}", sum, 0, start.elapsed());
}

fn expand_sky(sky: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut big_sky: Vec<Vec<char>> = vec![];

    for sky_line in sky {
        if sky_line.iter().all(|f| f != &'#') {
            big_sky.push(sky_line.clone());
        }
        big_sky.push(sky_line);
    }

    for index in (0..big_sky[0].len()).rev() {
        let mut has_galaxy = false;
        for big_sky_line in &big_sky {
            if big_sky_line[index] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            for big_sky_line in &mut big_sky {
                big_sky_line.insert(index, '.');
            }
        }
    }

    big_sky
}
