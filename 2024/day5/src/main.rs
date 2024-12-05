use std::{fs, time::Instant};

fn main() {
    let input =
        fs::read_to_string("./test_input.txt").expect("Should have been able to read the file");
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let (order_rules, page_updates) = input.split_once("\n\r\n").unwrap();

    let start = Instant::now();

    let order_rules = order_rules
        .split('\n')
        .map(|line| {
            let (from, to) = line.split_once('|').unwrap();

            (
                from.trim().parse::<i32>().unwrap().to_owned(),
                to.trim().parse::<i32>().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    let page_updates = page_updates
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|f| f.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Order Rules: {:?}", order_rules);
    println!("Page Updates: {:?}", page_updates);

    let mut sum = 0;
    let mut sum_2 = 0;

    for update in page_updates {
        let middle = update.len() / 2;

        assert!(update.len() % 2 == 1);
        let fixed = fix_updates(&order_rules, &update);

        if fixed == update {
            sum += update[middle];
        } else {
            sum_2 += fixed[middle];
        }
    }

    println!("Sum: {} Sum 2: {} took: {:?}", sum, sum_2, start.elapsed());
}

fn fix_updates(order_rules: &Vec<(i32, i32)>, page_updates: &Vec<i32>) -> Vec<i32> {
    let mut new_updates = page_updates.to_vec();

    new_updates.sort_by(|a, b| {
        let applied_rules_a = get_rules_for_number(order_rules, *a);
        if applied_rules_a.iter().any(|f| f.0 == *b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });
    new_updates.reverse();

    new_updates
}

fn get_rules_for_number(rules: &Vec<(i32, i32)>, number: i32) -> Vec<&(i32, i32)> {
    rules.iter().filter(|f| f.1 == number).collect::<Vec<_>>()
}
