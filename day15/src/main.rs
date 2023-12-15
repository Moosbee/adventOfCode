use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_parts = input.split(',');

    println!("Files Lines {}", input_parts.clone().count());

    let mut sum: i32 = 0;

    println!("Hash of 'HASH' is {}", gen_hash("HASH"));

    for part in input_parts {
        let hash: i32 = gen_hash(part).into();
        println!("Hash of '{}' is {}", part, hash);
        sum = sum + hash;
    }

    println!("Erg Part 1: {} Took {:?}", 0, start.elapsed())
}


fn gen_hash(text: &str) -> i32 {
  let mut sum = 0;
  for chr in text.chars() {
      let asci_code: i32 = chr as i32;
      sum = ((sum + asci_code) * 17) % 256;
  }
  sum
}
