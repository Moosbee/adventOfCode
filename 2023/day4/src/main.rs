use std::fs;

use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
pub struct Card {
    id: i32,
    // winning_numbers: Vec<i32>,
    // my_numbers: Vec<i32>,
    matching: i32,
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());
    let separator = Regex::new(r"([:|]+)").expect("Invalid regex");
    let spaces = Regex::new(r"(  +)").expect("Invalid regex");

    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    let cards: Vec<Card> = input_lines
        .filter_map(|line| {
            let small_line = spaces.replace_all(line, " ");
            let parts: Vec<&str> = separator.split(&small_line).into_iter().collect();

            if parts.len() == 3 {
                let id: i32 = parts[0]
                    .trim()
                    .strip_prefix("Card ")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default();

                let winning_numbers: Vec<i32> = parts[1]
                    .trim()
                    .split(" ")
                    .filter_map(|s| s.parse().ok())
                    .collect();
                let my_numbers: Vec<i32> = parts[2]
                    .trim()
                    .split(" ")
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let mut matching = 0;

                for my_number in my_numbers.clone() {
                    for win_number in winning_numbers.clone() {
                        if my_number == win_number {
                            matching = matching + 1;
                        }
                    }
                }

                return Some(Card {
                    id,
                    // winning_numbers,
                    // my_numbers,
                    matching,
                });
            }
            None
        })
        .collect();

    let indexing_duration = start.elapsed();

    let mut all_wins: Vec<i32> = vec![];

    for card in &cards {
        all_wins.push(card.id);
        all_wins.append(&mut collect_wins(card, &cards));
    }

    println!("Cards:{:?}", all_wins);
    println!("Cards len: {}", all_wins.len());
    let card_duration = start.elapsed();

    let total_score = count_score(&cards);

    let score_duration = start.elapsed();

    println!(
        "Final card count: {}, final score: {}",
        all_wins.len(),
        total_score
    );
    println!(
        "Durations: indexing: {:?}, card counting: {:?}, score counting: {:?}",
        indexing_duration, card_duration, score_duration
    );
}

fn count_score(cards: &Vec<Card>) -> i32 {
    let mut total_score = 0;

    for card in cards {
        let score: i32 = 2i32.pow(card.matching.try_into().unwrap_or_default()) / 2;
        println!("Id: {}, mat:{}, score:{}", card.id, card.matching, score);
        total_score += score
    }

    println!("Score: {}", total_score);
    total_score
}

fn collect_wins(card: &Card, cards: &Vec<Card>) -> Vec<i32> {
    let array_index: usize = (card.id - 1 + 1).try_into().unwrap_or_default();

    let matches: usize = card.matching.try_into().unwrap_or_default();

    let wins = &cards[array_index..array_index + matches];

    let mut all_wins: Vec<i32> = vec![];

    for win in wins {
        all_wins.push(win.id);
        all_wins.append(&mut collect_wins(win, cards));
    }

    println!("wins id {} wins {:?} all {:?}", card.id, wins, all_wins);

    all_wins
}
