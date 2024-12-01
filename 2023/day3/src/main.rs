use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    part1(&input);

    part2(&input);
}

fn part1(input: &String) {
    let input_lines: Vec<&str> = input.lines().collect();

    println!("Files Lines {}", input_lines.clone().len());

    let regex = Regex::new(r"[^.\d]").unwrap();

    println!("regex:{:?}", regex);

    let mut total = 0;

    for (index_line, chars) in input_lines.iter().enumerate() {
        let mut had_number = false;
        let mut start_index = 0;
        for (index_char, cha) in chars.chars().enumerate() {
            println!("Char:{} {} {}", cha, index_char, index_line);

            let is_num = cha.is_ascii_digit();

            if is_num && !had_number {
                start_index = index_char;
            }

            if had_number && !is_num {
                let number: i32 = chars[start_index..index_char]
                    .parse()
                    .ok()
                    .unwrap_or_default();

                println!("Sub-number:{}", number);

                let area_start = if start_index == 0 {
                    start_index
                } else {
                    start_index - 1
                };

                let area_end = chars.len().min(index_char + 1);

                let str_up;
                let str_middle = &chars[area_start..area_end];
                let str_down;

                if index_line != 0 {
                    str_up = &input_lines[index_line - 1][area_start..area_end];
                } else {
                    str_up = "";
                }

                if index_line != input_lines.len() - 1 {
                    str_down = &input_lines[index_line + 1][area_start..area_end];
                } else {
                    str_down = "";
                }

                let str_all = str_up.to_owned() + str_middle + str_down;

                println!(
                    "Area around it:\n{}\n{}\n{}\n{}",
                    &str_up,
                    &str_middle,
                    &str_down,
                    regex.is_match(&str_all)
                );

                if regex.is_match(&str_all) {
                    total += number;
                }

                println!("-------------------------------")
            }

            had_number = is_num;
        }
      }
      println!("Total: {}", total);
}

#[derive(Debug)]
pub struct Star {
    index_line: usize,
    index_char: usize,
    adjacent_numbers: Vec<i32>,
}

fn part2(input: &String) {
    let input_lines: Vec<&str> = input.lines().collect();

    println!("Files Lines {}", input_lines.clone().len());

    let regex = Regex::new(r"[\*]").unwrap();

    println!("regex:{:?}", regex);

    let mut stars: Vec<Star> = vec![];

    for (index_line, chars) in input_lines.iter().enumerate() {
        for (index_char, cha) in chars.chars().enumerate() {
            if cha == '*' {
                let new_star = Star {
                    index_line,
                    index_char,
                    adjacent_numbers: vec![],
                };
                stars.push(new_star);
            }
        }
    }

    println!("Stars:{:?}", &stars.len());

    for (index_line, chars) in input_lines.iter().enumerate() {
        let mut had_number = false;
        let mut start_index = 0;
        for (index_char, cha) in chars.chars().enumerate() {
            println!("Char:{} {} {}", cha, index_char, index_line);

            let is_num = cha.is_ascii_digit();

            if is_num && !had_number {
                start_index = index_char;
            }

            if had_number && !is_num {
                let number: i32 = chars[start_index..index_char]
                    .parse()
                    .ok()
                    .unwrap_or_default();

                let area_start = if start_index == 0 {
                    start_index
                } else {
                    start_index - 1
                };

                let area_end = chars.len().min(index_char + 1);

                let line_start = if index_line == 0 {
                    index_line
                } else {
                    index_line - 1
                };

                let line_end = chars.len().min(index_line + 1);

                for star in &mut stars {
                    println!(
                        "Num:{} near Star {} is {} than {} and {} than {}, {} is {} than {} and {} than {}",
                        number,
                        star.index_line,
                        star.index_line >= line_start,
                        line_start,
                        star.index_line <= line_end,
                        line_end,
                        star.index_char,
                        star.index_char >= area_start,
                        area_start,
                        star.index_char < area_end,
                        area_end
                    );

                    if star.index_line >= line_start
                        && star.index_line <= line_end
                        && star.index_char >= area_start
                        && star.index_char < area_end
                    {
                        star.adjacent_numbers.push(number);
                    }
                }
            }

            had_number = is_num;
        }
    }

    println!("All Stars:{}", &stars.len());

    let mut sum = 0;

    for str in stars {
      println!("Star {} {} has {} adjacent numbers",str.index_line,str.index_char,str.adjacent_numbers.len());
        if str.adjacent_numbers.len() == 2 {
            sum = sum + (str.adjacent_numbers[0] * str.adjacent_numbers[1]);
        }
    }

    println!("All gear ratios: {}", sum);
}
