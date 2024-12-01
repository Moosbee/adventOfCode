use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let mut end_number = 0;

    for lines in input_lines {
        let mut first_num = -1;
        let mut last_num = -1;

        for (index, lin) in lines.chars().enumerate() {
            match get_number(lin) {
                Some(num) => {
                    last_num = num;
                    if first_num == -1 {
                        first_num = num;
                    }
                }
                None => {}
            }
            let substring = &lines[index..];
            match get_number_text(substring) {
                Some(num) => {
                    last_num = num;
                    if first_num == -1 {
                        first_num = num;
                    }
                }
                None => {}
            }
            println!("Lengrer:{}", substring);
        }

        let final_num = first_num * 10 + last_num;

        end_number = end_number + final_num;

        println!("Num Line:{} {} {}", lines, final_num, end_number);
    }

    println!("The final number is: {}", end_number);
}

fn get_number(chr: char) -> Option<i32> {
    match chr {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

fn get_number_text(text: &str) -> Option<i32> {
    match text {
        s if s.starts_with("zero") => Some(0),
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ => None,
    }
}
