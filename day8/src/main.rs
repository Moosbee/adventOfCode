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

    let mut current_nodes: Vec<&Node> = nodes
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
    let mut times: u128 = 0;

    let start = Instant::now();

    loop {
        current_nodes = current_nodes
            .iter()
            .map(|node| get_next_node(node, turn_index, &nodes, &turn_signals))
            .collect();

        turn_index = (turn_index + 1) % turn_signals.len();

        times = times + 1;

        if times % 10000000 == 0 {
            let count = current_nodes
                .iter()
                .filter(|node| node.id.ends_with('Z'))
                .count();

            println!(
                "Node count: {} r nodes: {} Current Nodes:{:?} iteration:{:?}",
                current_nodes.len(),
                count,
                current_nodes,
                times
            );
        }

        // if count == current_nodes.len() {
        //     break;
        // }

        if current_nodes.iter().all(|node| node.id.ends_with('Z')) {
            break;
        }
    }

    println!(
        "Times: {} End: {:?} took: {:?}",
        times,
        current_nodes,
        start.elapsed()
    );
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
