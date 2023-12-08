use std::fs;

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

    nodes.sort();

    println!("Nodes:{:?}", nodes);

    let mut new_node_id: &String = &"AAA".to_string();
    let mut turn_index: usize = 0;
    let mut times = 0;

    loop {
        let new_node = nodes.iter().find(|f| f.id.eq(new_node_id)).unwrap();
        let turn = turn_signals[turn_index];
        turn_index = (turn_index + 1) % turn_signals.len();

        if turn == 'R' {
            new_node_id = &new_node.right;
        } else {
            new_node_id = &new_node.left;
        }

        println!(
            "Node_traverse: {:?} Turn: {} i: {} new node id: {} times: {}",
            new_node, turn, turn_index, new_node_id, times
        );
        if new_node.id == "ZZZ" {
            break;
        }
        times = times + 1;
    }

    println!("Times: {}", times);
}
