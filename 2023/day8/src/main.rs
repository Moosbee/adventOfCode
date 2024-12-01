use std::{fs, time::Instant};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let turn_signals: Vec<char> = input_lines.nth(0).unwrap().chars().collect();
    println!("turns:{:?}", turn_signals);
    input_lines.nth(0);
    println!("Files Lines {}", input_lines.clone().count());
    let mut nodes: Vec<Node> = input_lines
        .into_iter()
        .filter_map(|line| {
            if line.len() == 16 {
                Some(Node {
                    id: line[0..3].to_string(),
                    left: line[7..10].to_string(),
                    right: line[12..15].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    nodes.sort_by(|f, g| f.id.cmp(&g.id));

    println!("Nodes:{:?}", nodes);

    let current_nodes: Vec<&Node> = nodes
        .iter()
        .filter_map(|node| {
            if node.id.ends_with('A') {
                Some(node)
            } else {
                None
            }
        })
        .collect();

    println!("Current Nodes:{:?}", current_nodes);

    let mut turn_index: usize = 0;

    let start = Instant::now();

    let mut erg_vec: Vec<usize> = vec![];

    for current_node in current_nodes {
        let mut next_node = current_node;

        let mut times: usize = 0;

        loop {
            next_node = get_next_node(next_node, turn_index, &nodes, &turn_signals);

            turn_index = (turn_index + 1) % turn_signals.len();

            times = times + 1;

            if next_node.id.ends_with('Z') {
                break;
            }
        }
        erg_vec.push(times);
        println!(
            "Times: {} turn signals: {} ratio: {} mod: {} took: {:?}",
            times,
            turn_signals.len(),
            times / turn_signals.len(),
            times % turn_signals.len(),
            start.elapsed()
        );
    }

    println!(
        "Solution: {} took: {:?}",
        lcm_of_vector(erg_vec),
        start.elapsed()
    );
}

// Function to find the Greatest Common Divisor (GCD)
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to find the Least Common Multiple (LCM) of two numbers
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

// Function to find the LCM of a vector of integers
fn lcm_of_vector(numbers: Vec<usize>) -> usize {
    if numbers.is_empty() {
        panic!("Vector is empty");
    }

    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    result
}

fn get_next_node<'a>(
    old_node: &'a Node,
    turn_index: usize,
    nodes: &'a Vec<Node>,
    turn_signals: &'a Vec<char>,
) -> &'a Node {
    let turn = turn_signals[turn_index];

    let new_node_id: &String;

    if turn == 'R' {
        new_node_id = &old_node.right;
    } else {
        new_node_id = &old_node.left;
    }
    let new_node = nodes.binary_search_by(|f| f.id.cmp(new_node_id)).unwrap();

    // println!(
    //     "Node traverse from: {:?} to {:?} Turn: {} i: {} new node id: {} times: {}",
    //     old_node, new_node, turn, turn_index, new_node_id, times
    // );

    &nodes[new_node]
}
