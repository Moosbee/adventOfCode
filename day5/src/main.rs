use std::fs;

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file").replace("\r", "");

    let input_lines = input.split("\n\n");

    println!("Files Lines {}", input_lines.clone().count());
}
