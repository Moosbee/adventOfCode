use std::fs;

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let space_regex = Regex::new(r" +").unwrap();

    let reports = input_lines
        .map(|line| {
            let binding = space_regex.replace(line, " ");
            binding
                .trim()
                .split(" ")
                .map(|f| f.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Reports: {:?}", reports);

    let cout = reports
        .iter()
        .filter(|f| {
            let save = is_save(f.to_vec());
            println!("{:?} {}", f, save);
            save
        })
        .count();

    println!();
    println!();
    println!();

    let count2 = reports
        .iter()
        .filter(|f| {
            let save = is_save(f.to_vec());
            let save = if save {
                true
            } else {
                let new_vec = f.to_vec();

                for i in 0..new_vec.len() {
                    let mut new_vec2 = new_vec.clone();
                    new_vec2.remove(i);
                    let save2 = is_save(new_vec2);
                    if save2 {
                        return true;
                    }
                }

                false
            };
            println!("{:?} {}", f, save);
            save
        })
        .count();

    println!("Count: {} {}", cout, count2);
}

fn is_save(report: Vec<i32>) -> bool {
    let mut last = report[0];
    let up = (report[1] - report[0]) > 0;
    for rp in report.iter().skip(1) {
        let diff = rp - last;
        let this_up = diff > 0;
        println!("{} {} {} {} {}", up, this_up, last, rp, diff);
        if up != this_up || !(1..=3).contains(&diff.abs()) {
            return false;
        }

        last = *rp;
    }

    true
}
