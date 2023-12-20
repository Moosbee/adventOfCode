use core::fmt;
use std::collections::VecDeque;
use std::{collections::HashMap, fs};

use std::time::Instant;

use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::EdgeRef;

#[derive(Debug, Clone, Hash)]
struct Module {
    m_type: ModuleType,
    id: String,
    state: bool,
    next_ids: Vec<String>,
}
#[derive(Debug, Clone, Copy, Hash)]
enum ModuleType {
    /// Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
    FlipFlop,
    /// Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
    Conjunction,
    /// There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.
    Broadcast,
    /// Output for testing purposes, does nothing
    Output,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:?}: {}", self.id, self.m_type, self.state)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge(usize, bool);

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct ModuleUpdate {
    old_module_node: NodeIndex,
    new_module_node: NodeIndex,
    state: bool,
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let input_lines = input.lines();

    println!("Files Lines {}", input_lines.clone().count());

    let mut module_graph: DiGraph<Module, Edge> = DiGraph::new();
    let mut module_hashmap: HashMap<String, NodeIndex> = HashMap::new();

    for line in input_lines {
        let module = parse_module(line).expect("Should have been able parse line");
        let id = module.id.clone();
        let module_id = module_graph.add_node(module);
        module_hashmap.insert(id, module_id);
    }

    if !module_hashmap.contains_key("output") {
        let module_id = module_graph.add_node(Module {
            m_type: ModuleType::Output,
            id: "output".to_owned(),
            state: false,
            next_ids: vec![],
        });
        module_hashmap.insert("output".to_owned(), module_id);
    }

    println!(
        "Graph: {:?} start {:?}",
        module_graph,
        module_graph.node_weight(*module_hashmap.get("broadcaster").unwrap())
    );

    for module_id in &module_hashmap {
        let module = module_graph.node_weight(*module_id.1).unwrap().clone();
        for next_id in module.next_ids.iter().enumerate() {
            let next_vertex: Option<&NodeIndex> = module_hashmap.get(next_id.1);
            println!(
                "vertex name: {:?} id: {:?} next {:?}",
                module_id.0, module_id.1, next_vertex
            );
            if next_vertex.is_some() {
                module_graph.add_edge(*module_id.1, *next_vertex.unwrap(), Edge(next_id.0, false));
            } else {
                module_graph.add_edge(
                    *module_id.1,
                    *module_hashmap.get("output").unwrap(),
                    Edge(next_id.0, false),
                );
            }
        }
    }

    println!(
        "\n{} \n can be displayed in https://viz-js.com/",
        Dot::new(&module_graph)
    );

    let mut update_queue: VecDeque<ModuleUpdate> = VecDeque::new();

    let mut true_count = 0;
    let mut false_count = 0;

    let broadcaster_node = *module_hashmap.get("broadcaster").unwrap();
    // let output_node = *module_hashmap.get("output").unwrap();

    let mut stop = false;
    let mut index = 0;
    for _index in 0..10000 {
        if stop {
            break;
        }
        index += 1;// 244.055.946.148.853

        update_queue.push_back(ModuleUpdate {
            old_module_node: broadcaster_node,
            new_module_node: broadcaster_node,
            state: false,
        });

        while !update_queue.is_empty() {
            let next_update_opt = update_queue.pop_front();
            if next_update_opt.is_some() {
                let next_update = next_update_opt.unwrap();
                match next_update.state {
                    true => {
                        true_count += 1;
                    }
                    false => {
                        false_count += 1;
                    }
                }
                stop = stop
                    || update_module(
                        next_update.old_module_node,
                        next_update.new_module_node,
                        next_update.state,
                        index,
                        &mut module_graph,
                        &mut update_queue,
                    );
            }
        }
    }
    println!(
        "\nAfter {} turns\n{} \n can be displayed in https://viz-js.com/\n",
        10000,
        Dot::new(&module_graph)
    );

    println!(
        "Solution: true {} * false {} = {} Took {:?}",
        true_count,
        false_count,
        true_count * false_count,
        start.elapsed()
    )
}

fn update_module(
    old_module_node: NodeIndex,
    new_module_node: NodeIndex,
    state: bool,
    run_count: i32,
    module_graph: &mut DiGraph<Module, Edge>,
    update_queue: &mut VecDeque<ModuleUpdate>,
) -> bool {
    {
        let edges: Vec<_>;
        {
            let edges_borrow: Vec<petgraph::graph::EdgeReference<'_, Edge>> = module_graph
                .edges_connecting(old_module_node, new_module_node)
                .collect();

            edges = edges_borrow.iter().map(|f| f.id().clone()).collect();
        }

        for edge_index in edges {
            let edge = module_graph.edge_weight_mut(edge_index);
            if edge.is_some() {
                let sm_edge = edge.unwrap();
                sm_edge.1 = state;
            }
        }
    }
    let module_opt = module_graph.node_weight(new_module_node);

    let module = module_opt.expect("Should exist");
    // println!(
    //     "{} -{}-> {}",
    //     module_graph.node_weight(old_module_node).unwrap().id,
    //     state,
    //     module.id
    // );
    match module.m_type {
        ModuleType::FlipFlop => {
            update_flip_flop_module(new_module_node, state, module_graph, update_queue);
            false
        }
        ModuleType::Conjunction => {
            update_conjunction_module(new_module_node, module_graph, update_queue);
            false
        }
        ModuleType::Broadcast => {
            update_broadcast_module(new_module_node, state, module_graph, update_queue);
            false
        }
        ModuleType::Output => update_output_module(
            old_module_node,
            new_module_node,
            state,
            run_count,
            module_graph,
            update_queue,
        ),
    }
}

fn update_flip_flop_module(
    new_module_node: NodeIndex,
    state: bool,
    module_graph: &mut DiGraph<Module, Edge>,
    update_queue: &mut VecDeque<ModuleUpdate>,
) {
    // println!("FlipFlop reached with value {}", state);
    if !state {
        let new_state: bool;
        {
            let module = module_graph.node_weight_mut(new_module_node).unwrap();
            module.state = !module.state;
            new_state = module.state;
        }

        let out_list: Vec<NodeIndex> = module_graph
            .neighbors_directed(new_module_node, petgraph::Direction::Outgoing)
            .collect();

        for out in out_list.iter().rev() {
            update_queue.push_back(ModuleUpdate {
                old_module_node: new_module_node,
                new_module_node: *out,
                state: new_state,
            });
            // update_module(new_module_node, *out, new_state, module_graph, update_queue);
        }
    }
}
fn update_conjunction_module(
    new_module_node: NodeIndex,
    module_graph: &mut DiGraph<Module, Edge>,
    update_queue: &mut VecDeque<ModuleUpdate>,
) {
    // println!("Conjunction reached with value {}", state);
    let mut is_all_true = true;
    {
        let before_nodes =
            module_graph.edges_directed(new_module_node, petgraph::Direction::Incoming);
        for node in before_nodes {
            is_all_true = is_all_true && node.weight().1;
            if !is_all_true {
                break;
            }
        }
    }
    let out_list: Vec<NodeIndex> = module_graph
        .neighbors_directed(new_module_node, petgraph::Direction::Outgoing)
        .collect();
    for out in out_list.iter().rev() {
        update_queue.push_back(ModuleUpdate {
            old_module_node: new_module_node,
            new_module_node: *out,
            state: !is_all_true,
        });
        // update_module(
        //     new_module_node,
        //     *out,
        //     !is_all_true,
        //     module_graph,
        //     update_queue,
        // );
    }
}
fn update_broadcast_module(
    new_module_node: NodeIndex,
    state: bool,
    module_graph: &mut DiGraph<Module, Edge>,
    update_queue: &mut VecDeque<ModuleUpdate>,
) {
    // println!("Broadcast reached with value {}", state);
    let out_list: Vec<NodeIndex> = module_graph
        .neighbors_directed(new_module_node, petgraph::Direction::Outgoing)
        .collect();

    for out in out_list.iter().rev() {
        update_queue.push_back(ModuleUpdate {
            old_module_node: new_module_node,
            new_module_node: *out,
            state,
        });
        // update_module(new_module_node, *out, state, module_graph, update_queue);
    }
}
fn update_output_module(
    old_module_node: NodeIndex,
    _new_module_node: NodeIndex,
    _state: bool,
    _run_count: i32,
    _module_graph: &mut DiGraph<Module, Edge>,
    _update_queue: &mut VecDeque<ModuleUpdate>,
) -> bool {
    let old_module = _module_graph.node_weight(old_module_node).unwrap();
    match &old_module.id[..] {
        "dh" => {
            if _state {
                println!(
                    "Output reached from {} after {} round",
                    old_module, _run_count
                );
            }
            false
        }
        "dp" => {
            if _state {
                println!(
                    "Output reached from {} after {} round",
                    old_module, _run_count
                );
            }
            false
        }
        "bb" => {
            if _state {
                println!(
                    "Output reached from {} after {} round",
                    old_module, _run_count
                );
            }
            false
        }
        "qd" => {
            if _state {
                println!(
                    "Output reached from {} after {} round",
                    old_module, _run_count
                );
            }

            false
        }
        "rm" => {
            if !_state {
                println!(
                    "Output reached from {} after {} round",
                    old_module, _run_count
                );
            }
            false
        }
        _ => {
            println!(
                "Output reached from {:?} with value {} after {} round",
                old_module, _state, _run_count
            );
            false
        }
    }
}

fn parse_module(line: &str) -> Result<Module, String> {
    let split = line.split_once(" -> ");
    if split.is_none() {
        return Err("Error, no split:".to_owned() + line);
    }
    let (id_type, next_nodes_str) = split.unwrap();

    let next_nodes: Vec<String> = next_nodes_str.split(", ").map(|f| f.to_owned()).collect();

    let erg: Result<(String, ModuleType), String> = match (&id_type[..1], &id_type[1..]) {
        ("%", _) => Ok((id_type[1..].to_owned(), ModuleType::FlipFlop)),
        ("&", _) => Ok((id_type[1..].to_owned(), ModuleType::Conjunction)),
        ("b", "roadcaster") => Ok(("broadcaster".to_owned(), ModuleType::Broadcast)),
        ("o", "utput") => Ok(("output".to_owned(), ModuleType::Output)),
        _ => Err("Error, no match:".to_owned() + &id_type[..1] + " " + &id_type[1..] + " " + line),
    };

    if erg.is_err() {
        return Err(erg.err().unwrap());
    }

    let (id, m_type) = erg.unwrap();

    Ok(Module {
        m_type,
        id,
        state: false,
        next_ids: next_nodes,
    })
}
