use std::time::Instant;
use std::{fs, i32};

#[derive(Debug)]
pub struct Hand {
    hand: [i32; 5],
    hand_type: HandType,
    bid: i32,
}

#[derive(Debug, PartialEq, PartialOrd)]

enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}
fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut hands: Vec<Hand> = vec![];

    for line in input_lines {
        let mut hand_line = line.split(" ");
        if hand_line.clone().count() == 2 {
            let hand_text = hand_line.nth(0).unwrap();
            let hand_bid: i32 = hand_line.nth(0).and_then(|s| s.parse().ok()).unwrap();

            let hand_numbers: [i32; 5] = (hand_text
                .chars()
                .filter_map(get_number)
                .collect::<Vec<i32>>())
            .try_into()
            .unwrap();

            let hand = Hand {
                bid: hand_bid,
                hand: hand_numbers,
                hand_type: calc_hand_type(hand_numbers),
            };

            hands.push(hand);
        }
    }

    println!("hands:{:?}", hands);

    hands.sort_by(|hand_a, hand_b| {
        if hand_a.hand_type > hand_b.hand_type {
            return std::cmp::Ordering::Greater;
        } else if hand_a.hand_type < hand_b.hand_type {
            return std::cmp::Ordering::Less;
        }

        for i in 0..5 {
            if hand_a.hand[i] > hand_b.hand[i] {
                return std::cmp::Ordering::Greater;
            } else if hand_a.hand[i] < hand_b.hand[i] {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    });

    println!("hands:{:?}", hands);

    let mut total: i32 = 0;

    for (index, hand) in hands.iter().enumerate() {
        let score: i32 = index.try_into().unwrap();
        total += (score + 1) * hand.bid;
    }

    println!("Solution: {} took {:?}", total, start.elapsed());
}

fn calc_hand_type(hand_numbers: [i32; 5]) -> HandType {
    let mut numbers: Vec<(i32, i32)> = vec![];

    for number in hand_numbers {
        let mut is_in = false;
        for num in &mut numbers {
            if num.1 == number {
                is_in = true;
                num.0 += 1;
            }
        }
        if !is_in {
            numbers.push((1, number))
        }
    }

    numbers.sort();

    let best = numbers
        .iter()
        .filter(|f| f.1 != 1)
        .last()
        .unwrap_or(&(1, 13))
        .clone()
        .1;

    numbers = vec![];

    for mut number in hand_numbers {
        if number == 1 {
            number = best;
        }
        let mut is_in = false;
        for num in &mut numbers {
            if num.1 == number {
                is_in = true;
                num.0 += 1;
            }
        }
        if !is_in {
            numbers.push((1, number))
        }
    }

    numbers.sort();

    let num_pairs: Vec<i32> = numbers.iter().map(|f| f.0).rev().collect();

    match num_pairs[..] {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => HandType::HighCard,
    }
}

fn get_number(chr: char) -> Option<i32> {
    match chr {
        'J' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'Q' => Some(11),
        'K' => Some(12),
        'A' => Some(13),
        _ => None,
    }
}
