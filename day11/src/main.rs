use std::fs;

use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let galaxies: Vec<Galaxy> = get_galaxies(
        input_lines.map(|line| line.chars().collect()).collect(),
        1000000,
    );

    println!("Galaxies: {:?}", galaxies.len());

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

    println!("Galaxy Pairs: {:?}", galaxy_pairs.len());

    let mut sum: u128 = 0;

    for pair in galaxy_pairs {
        let distance = ((pair.0.x as i128) - (pair.1.x as i128)).abs()
            + ((pair.0.y as i128) - (pair.1.y as i128)).abs();
        sum = sum + (distance as u128);
        // println!("Galaxy Pair: {:?} {}", pair, distance);
    }

    println!("Part 2: {} took {:?}", sum, start.elapsed());
}

fn get_galaxies(sky: Vec<Vec<char>>, size_factor: usize) -> Vec<Galaxy> {
    let mut empty_lines: Vec<usize> = vec![];

    for sky_line in sky.iter().enumerate() {
        if sky_line.1.iter().all(|f| f != &'#') {
            empty_lines.push(sky_line.0);
        }
    }

    let mut empty_rows: Vec<usize> = vec![];

    for index in 0..sky[0].len() {
        let mut has_galaxy = false;
        for big_sky_line in &sky {
            if big_sky_line[index] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            empty_rows.push(index);
        }
    }

    println!("empty lines:{:?} empty rows:{:?}", empty_lines, empty_rows);

    let mut galaxies: Vec<Galaxy> = vec![];

    for sky_line in sky.iter().enumerate() {
        for sky_grid in sky_line.1.iter().enumerate() {
            if sky_grid.1 == &'#' {
                let mut line_count = 0;
                for line in &empty_lines {
                    if line > &sky_line.0 {
                        break;
                    }
                    line_count = line_count + 1;
                }
                let mut row_count = 0;
                for row in &empty_rows {
                    if row > &sky_grid.0 {
                        break;
                    }
                    row_count = row_count + 1;
                }

                galaxies.push(Galaxy {
                    x: sky_line.0 + line_count * (size_factor - 1),
                    y: sky_grid.0 + row_count * (size_factor - 1),
                })
            }
        }
    }

    galaxies
}
