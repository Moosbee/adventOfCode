use std::{collections::VecDeque, fs, time::Instant};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_chars = input.trim().chars();

    println!("Files Lines {}", input_chars.clone().count());

    let start = Instant::now();

    let mut expanded_list: VecDeque<Option<i32>> = VecDeque::new();

    let mut empty = false;

    let mut id = 0;
    println!("Starting unfolding");
    for c in input_chars {
        let count = c.to_string().parse::<i32>().unwrap();

        for _ in 0..count {
            expanded_list.push_back(if !empty { Some(id) } else { None });
        }
        if empty {
            id += 1;
        }
        empty = !empty;
    }

    println!("Finished unfolding");

    // for i in expanded_list.iter() {
    //     print!("{}", i.map(|f| f.to_string()).unwrap_or(".".to_string()));
    // }
    println!();
    println!("Compressing");

    let compressed = compress_list_easy(expanded_list.clone());

    // for i in compressed.iter() {
    //     print!("{}", i.map(|f| f.to_string()).unwrap_or(".".to_string()));
    // }
    println!();

    println!("Done Compressing");

    let part_1 = checksum(compressed);

    // println!("Part 1: {}", part_1);

    println!("Compressing Hard");

    let compressed = compress_list_hard(expanded_list.clone(), id);

    // for i in compressed.iter() {
    //     print!("{}", i.map(|f| f.to_string()).unwrap_or(".".to_string()));
    // }
    println!();

    println!("Done Compressing Hard");

    let part_2 = checksum(compressed);

    println!(
        "Part 1: {} Part 2: {} Took {:?}",
        part_1,
        part_2,
        start.elapsed()
    );
}

fn compress_list_easy(mut list: VecDeque<Option<i32>>) -> Vec<Option<i32>> {
    let mut compressed = Vec::new();

    while let Some(x) = list.pop_front() {
        if let Some(x) = x {
            compressed.push(Some(x));
        } else {
            let last = get_last_id(&mut list);
            if let Some(last) = last {
                compressed.push(Some(last));
            } else {
                break;
            }
        }
    }

    compressed
}

fn get_last_id(list: &mut VecDeque<Option<i32>>) -> Option<i32> {
    while let Some(x) = list.pop_back() {
        if let Some(x) = x {
            return Some(x);
        }
    }
    None
}

fn compress_list_hard(mut list: VecDeque<Option<i32>>, max_id: i32) -> Vec<Option<i32>> {
    for i in (0..=max_id).rev() {
        let range = get_range(&list, i);
        if range.is_none() {
            continue;
        }
        let range = range.unwrap();
        let distance = range.1 - range.0;
        let mut last = 0;
        for j in 0..range.0 {
            let cuurent = list[j];
            if cuurent.is_some() {
                last = j;
            }
            if j - last > distance {
                let last = last + 1;
                for l in range.0..=range.1 {
                    let index = last + (l - range.0);
                    list[index] = list[l];
                    list[l] = None;
                }
                break;
            }
        }
    }

    list.into_iter().collect()
}

fn get_range(list: &VecDeque<Option<i32>>, id: i32) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;
    for (i, x) in list.iter().enumerate() {
        if let Some(x) = x {
            if *x == id {
                if start.is_none() {
                    start = Some(i);
                }
                end = Some(i);
            }
        }
    }
    start.zip(end)
}

fn checksum(list: Vec<Option<i32>>) -> i128 {
    list.iter()
        .enumerate()
        .map(|f| f.0 as i128 * *f.1.as_ref().unwrap_or(&0) as i128)
        .sum()
}
