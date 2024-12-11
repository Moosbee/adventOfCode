use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.split('\n');

    println!("Files Lines {}", input_lines.clone().count());

    let start = Instant::now();

    let stones = input
        .trim()
        .split(" ")
        .map(|f| f.trim().parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let total_turns = 25;
    // let total_turns = 1;

    println!("Start Stones: {:?}", stones);

    let mut total_stones_25 = 0;

    let mut cache = HashMap::new();

    for stone in stones.clone().into_iter() {
        let stones = gen_stones_count(stone, 0, total_turns, &mut cache);
        println!("Stone: {} Stones: {:?}", stone, stones);
        total_stones_25 = total_stones_25 + stones;
    }

    let mut total_stones_75 = 0;

    let total_turns = 75;

    for stone in stones.clone().into_iter() {
        let stones = gen_stones_count(stone, 0, total_turns, &mut cache);
        println!("Stone: {} Stones: {:?}", stone, stones);
        total_stones_75 = total_stones_75 + stones;
    }

    println!(
        "Part 1: {}, Part 2: {}, Took {:?}",
        total_stones_25,
        total_stones_75,
        start.elapsed()
    );
}

fn gen_stones_count_cached(
    stone: i128,
    turn: i128,
    max_turn: i128,
    cache: &mut HashMap<(i128, i128, i128), i128>,
) -> i128 {
    if !cache.contains_key(&(stone, turn, max_turn)) {
        let stones = gen_stones_count(stone, turn, max_turn, cache);
        cache.insert((stone, turn, max_turn), stones);
    }
    *cache.get(&(stone, turn, max_turn)).unwrap()
}

fn gen_stones_count(
    stone: i128,
    turn: i128,
    max_turn: i128,
    cache: &mut HashMap<(i128, i128, i128), i128>,
) -> i128 {
    let turn = turn + 1;
    if turn > max_turn {
        // println!("Turn: {} Stone: {}", turn, stone);
        return 1;
    }
    if stone == 0 {
        return gen_stones_count_cached(1, turn, max_turn, cache);
    }

    let length = stone.ilog10() + 1;

    if length % 2 == 0 {
        let stone_1 = stone / ((10 as i128).pow(length / 2));
        let stone_2 = stone % ((10 as i128).pow(length / 2));

        let erg = (
            gen_stones_count_cached(stone_1, turn, max_turn, cache),
            gen_stones_count_cached(stone_2, turn, max_turn, cache),
        );
        erg.0 + erg.1
    } else {
        gen_stones_count_cached(stone * 2024, turn, max_turn, cache)
    }
}
