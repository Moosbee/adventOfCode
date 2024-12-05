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
        let is_in_order = is_update_in_order(&order_rules, &update);
        let middle = update.len() / 2;
        println!(
            "Update: {:?} Is in order: {} Middle: {} length: {}",
            update,
            is_in_order,
            middle,
            update.len()
        );

        assert!(update.len() % 2 == 1);

        if is_in_order {
            sum += update[middle];
        } else {
            let fixed = fix_updates(&order_rules, &update);
            println!(
                "Fixed: {:?} Is in order: {}",
                fixed,
                is_update_in_order(&order_rules, &fixed)
            );
            sum_2 += fixed[middle];
        }
    }

    println!("Sum: {} Sum 2: {} took: {:?}", sum, sum_2, start.elapsed());
}

fn is_update_in_order(order_rules: &Vec<(i32, i32)>, page_updates: &Vec<i32>) -> bool {
    for i in 0..page_updates.len() {
        // println!("{} {}", page_updates[i], i);

        let applied_rules = get_rules_for_number(order_rules, page_updates[i])
            .iter()
            .map(|f| f.0)
            .collect::<Vec<_>>();

        for j in 0..i {
            if applied_rules.contains(&page_updates[j]) {
                // println!("ye {} {}", page_updates[j], j);
            } else {
                // println!("no {} {}", page_updates[j], j);
                return false;
            }
        }
        for j in i..page_updates.len() {
            if applied_rules.contains(&page_updates[j]) {
                // println!("no {} {}", page_updates[j], j);
                return false;
            } else {
                // println!("ye {} {}", page_updates[j], j);
            }
        }
    }

    true
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
