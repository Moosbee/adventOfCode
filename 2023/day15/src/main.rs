use std::fs;
use std::time::Instant;

struct LenseBox {
    lenses: Vec<Lense>,
}
struct Lense {
    id: String,
    focal_power: i8,
}

fn main() {
    let start = Instant::now();
    let input =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_parts = input.split(',');

    println!("Files Lines {}", input_parts.clone().count());

    let mut check_sum: i32 = 0;

    println!("Hash of 'HASH' is {}", gen_hash("HASH"));

    let mut lense_boxes: Vec<LenseBox> = (0..256).map(|_f| LenseBox { lenses: vec![] }).collect();

    for part in input_parts {
        let hash: usize = gen_hash(part).into();
        // println!("Hash of '{}' is {}", part, hash);
        check_sum = check_sum + (hash as i32);

        if part.ends_with('-') {
            let lense_name = &part[0..part.len() - 1];
            // println!("Lense_name: {}", lense_name);
            let lense_box = &mut lense_boxes[gen_hash(lense_name)];
            let pos = lense_box.lenses.iter().position(|f| f.id == lense_name);
            if pos.is_some() {
                lense_box.lenses.remove(pos.unwrap());
            }
        } else if part.contains('=') {
            let (lense_name, focal_power_str) = part.split_once('=').unwrap();
            let lense_box = &mut lense_boxes[gen_hash(lense_name)];
            let focal_power: i8 = focal_power_str.parse().unwrap();
            // println!("Lense name: {} power: {}", lense_name, focal_power);
            let pos = lense_box.lenses.iter().position(|f| f.id == lense_name);
            if pos.is_some() {
                lense_box.lenses[pos.unwrap()] = Lense {
                    id: lense_name.to_string(),
                    focal_power,
                }
            } else {
                lense_box.lenses.push(Lense {
                    id: lense_name.to_string(),
                    focal_power,
                });
            }
        }
        // print_boxes(&lense_boxes);
    }
        print_boxes(&lense_boxes);

    println!(
        "Erg Part 1: {} Part 2: {} Took {:?}",
        check_sum,
        calc_lense_powers(&lense_boxes),
        start.elapsed()
    )
}

fn print_boxes(boxes: &Vec<LenseBox>) {
    for (index, lense_box) in boxes.iter().enumerate() {
        if lense_box.lenses.len() != 0 {
            print!("Box {}:", index);
            for lense in &lense_box.lenses {
                print!(" [{} {}]", lense.id, lense.focal_power);
            }
            println!();
        }
    }
}

fn calc_lense_powers(boxes: &Vec<LenseBox>) -> i32 {
    let mut sum = 0;
    for (box_index, lense_box) in boxes.iter().enumerate() {
        for (lense_index, lense) in lense_box.lenses.iter().enumerate() {
            let lense_power = (box_index + 1) * (lense_index + 1) * (lense.focal_power as usize);
            sum = sum + lense_power as i32;
        }
    }
    sum
}

fn gen_hash(text: &str) -> usize {
    let mut sum = 0;
    for chr in text.chars() {
        let asci_code: usize = chr as usize;
        sum = ((sum + asci_code) * 17) % 256;
    }
    sum
}
