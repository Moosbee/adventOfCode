use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let mut end_number=0;

    for lines in input_lines {
        let mut first_num = -1;
        let mut last_num = -1;

        for lin in lines.chars() {
            match get_number(lin) {
                Some(num) => {
                    last_num=num;
                    if first_num==-1 {
                        first_num=num;
                    }
                },
                None => {},
            }
        }

        let final_num=first_num*10+last_num;

        end_number=end_number+final_num;

        println!("Num Line:{} {} {}",lines,final_num,end_number);
    }

    println!("The final number is: {}",end_number);
}

fn get_number(chr: char) -> Option<i32> {
    match chr {
        '0' => {Some(0)}
        '1' => {Some(1)}
        '2' => {Some(2)}
        '3' => {Some(3)}
        '4' => {Some(4)}
        '5' => {Some(5)}
        '6' => {Some(6)}
        '7' => {Some(7)}
        '8' => {Some(8)}
        '9' => {Some(9)}
        _ => {None}
    }
}
