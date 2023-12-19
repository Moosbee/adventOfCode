use std::collections::HashMap;
use std::fs;

use std::time::Instant;

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug)]
struct WorkFlow {
    id: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    categories: PartType,
    com: Compare,
    value: i32,
    next_id: String,
    default: bool,
}

#[derive(Debug)]
enum Compare {
    Bigger,
    Smaller,
}

#[derive(Debug)]
enum PartType {
    X,
    M,
    A,
    S,
}

fn main() {
    let start_time = Instant::now();
    let input = fs::read_to_string("./test_input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let input_lines = input.split_once("\n\n").unwrap();

    let rules_vec: Vec<WorkFlow> = input_lines
        .0
        .lines()
        .filter_map(|line| {
            let (id, rule_str) = line.split_once("{").unwrap();
            let rules_spl = rule_str.trim_end_matches("}").split(",");
            let mut rules: Vec<Rule> = vec![];
            for rule_set in rules_spl {
                let split_rules: Vec<&str> = rule_set.split(':').collect();
                if split_rules.len() == 1 {
                    rules.push(Rule {
                        categories: PartType::A,
                        com: Compare::Bigger,
                        value: 0,
                        next_id: split_rules[0].to_string(),
                        default: true,
                    })
                } else if split_rules.len() == 2 {
                    let mut compare = split_rules[0].chars();
                    let part_type = compare.next().unwrap();
                    let compar_op = compare.next().unwrap();
                    drop(compare);
                    let value: i32 = split_rules[0][2..].parse().unwrap();
                    rules.push(Rule {
                        categories: match part_type {
                            'x' => PartType::X,
                            'm' => PartType::M,
                            'a' => PartType::A,
                            's' => PartType::S,
                            _ => PartType::A,
                        },
                        com: match compar_op {
                            '>' => Compare::Bigger,
                            '<' => Compare::Smaller,
                            _ => Compare::Bigger,
                        },
                        value,
                        next_id: split_rules[1].to_string(),
                        default: false,
                    })
                }
            }

            Some(WorkFlow {
                id: id.to_string(),
                rules,
            })
        })
        .collect();

    let parts: Vec<Part> = input_lines
        .1
        .lines()
        .filter_map(|line| {
            let mut part: Part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };

            for text in line.trim_matches(|c| c == '{' || c == '}').split(',') {
                let (name, value) = text.split_once('=').unwrap();
                match name {
                    "a" => part.a = value.parse().unwrap(),
                    "m" => part.m = value.parse().unwrap(),
                    "s" => part.s = value.parse().unwrap(),
                    "x" => part.x = value.parse().unwrap(),
                    _ => {}
                }
            }

            Some(part)
        })
        .collect();

    let mut hash_rules: HashMap<String, Vec<Rule>> = HashMap::new();

    let rule_len = rules_vec.len();

    for workflow in rules_vec {
        hash_rules.insert(workflow.id, workflow.rules);
    }

    println!(
        "Files Lines {} {} Start: {:?}",
        rule_len,
        parts.len(),
        hash_rules.get("in")
    );

    // for rule in rules {
    //     println!("{:?}", rule);
    // }
    // for part in parts {
    //     println!("{:?}", part);
    // }

    let filtered: Vec<&Part> = parts
        .iter()
        .filter(|f| resolve_part(&f, &hash_rules, "in".to_string()))
        .collect();

    let mut total = 0;

    for filtered_part in filtered {
        let score = filtered_part.a + filtered_part.m + filtered_part.s + filtered_part.x;
        total = total + score;
        println!("Score: {} {:?}", score, filtered_part);
    }

    println!("Solution: {} Took {:?}", total, start_time.elapsed());

    let mut total: u64 = 0;

    for x_i in (1..4001).step_by(1) {
        for m_i in (1..4001).step_by(1) {
            for a_i in (1..4001).step_by(1) {
                for s_i in (1..4001).step_by(1) {
                    if resolve_part(
                        &Part {
                            x: x_i,
                            m: m_i,
                            a: a_i,
                            s: s_i,
                        },
                        &hash_rules,
                        "in".to_string(),
                    ) {
                        total = total + 1;
                    }
                }
            }
            println!("Total: {}", total)
        }
    }

    println!("Solution: {} Took {:?}", total, start_time.elapsed());
}

fn resolve_part(part: &Part, workflows: &HashMap<String, Vec<Rule>>, start: String) -> bool {
    let mut next_rule = start;
    let accepted = loop {
        next_rule = resolve_workflow(&part, workflows.get(&next_rule).unwrap());
        if next_rule == "R" {
            break false;
        } else if next_rule == "A" {
            break true;
        }
    };

    accepted
}

fn resolve_workflow(part: &Part, rules: &Vec<Rule>) -> String {
    for rule in rules {
        let comp_value = match rule.categories {
            PartType::A => part.a,
            PartType::M => part.m,
            PartType::S => part.s,
            PartType::X => part.x,
        };
        let applies = match rule.com {
            Compare::Bigger => comp_value > rule.value,
            Compare::Smaller => comp_value < rule.value,
        };
        if applies || rule.default {
            return rule.next_id.clone();
        }
    }

    "R".to_string()
}
