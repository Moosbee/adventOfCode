use std::fs;

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let instructions = mul_regex
        .find_iter(&input)
        .map(|f| f.as_str())
        .collect::<Vec<_>>();

    println!("Erg: {:?}", instructions);

    let mut sum = 0;

    for instruction in instructions {
        if mul_regex.is_match(instruction) {
            let (first, second) = instruction.split_once(',').unwrap();
            let num1 = first.replace("mul(", "").parse::<i32>().unwrap();
            let num2 = second.replace(")", "").parse::<i32>().unwrap();
            let erg = num1 * num2;
            sum = sum + erg;
            println!("{} * {} = {} {}", num1, num2, erg, sum);
        }
    }

    println!("Sum: {}", sum);
}
