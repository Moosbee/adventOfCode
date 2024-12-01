fn main() {
  let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

  let input_lines = input.split('\n');

  println!("Files Lines {}", input_lines.clone().count());

  
}
