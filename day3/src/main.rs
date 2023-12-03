use std::fs;
use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines: Vec<&str> = input.lines().collect();

    println!("Files Lines {}", input_lines.clone().len());

    let regex = Regex::new(r"[^.\d]").unwrap();

    println!("regex:{:?}",regex);

    // let line_chars=input_lines.map(|x| x.chars());

    let mut total=0;

    for (index_line, chars) in input_lines.iter().enumerate() {
        let mut had_number = false;
        let mut start_index = 0;
        for (index_char, cha) in chars.chars().enumerate() {
            // println!("Char:{} {} {}", cha, index_char, index_line);

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

                let str_all= str_up.to_owned()+str_middle+str_down;

                println!(
                    "Area around it:\n{}\n{}\n{}\n{}",
                    &str_up, &str_middle, &str_down,regex.is_match(&str_all)
                );

                if regex.is_match(&str_all) {
                    total += number;
                }

                println!("-------------------------------")

            }

            had_number = is_num;
        }

        println!("Total: {}",total);
    }
}
