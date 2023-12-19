use std::collections::HashMap;
use std::fs;

use std::time::Instant;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

fn calc_size(ranges: &PartRange) -> u64 {
    let mut out = 1;
    out *= ranges.x.1 as u64 - ranges.x.0 as u64 + 1;
    out *= ranges.m.1 as u64 - ranges.m.0 as u64 + 1;
    out *= ranges.a.1 as u64 - ranges.a.0 as u64 + 1;
    out *= ranges.s.1 as u64 - ranges.s.0 as u64 + 1;
    out
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
    value: u32,
    next_id: String,
    default: bool,
}

#[derive(Debug, Clone, Copy)]
enum Compare {
    Bigger,
    Smaller,
}

#[derive(Debug, Clone, Copy)]
enum PartType {
    X,
    M,
    A,
    S,
}

fn main() {
    let start_time = Instant::now();
    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let (hash_rules, parts) = parse_input(&input);

    println!(
        "Files Lines {} Start: {:?}",
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

    let entire_range = PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let part_2 = get_workflow_count(entire_range, &hash_rules, "in".to_owned());

    // Expected 167409079868000
    // Got      167409079868000
    // Total    256000000000000

    // Got      143760172569135

    println!("Solution: {} Took {:?}", part_2, start_time.elapsed());
}

fn get_workflow_count(
    range: PartRange,
    workflows: &HashMap<String, Vec<Rule>>,
    workflow_id: String,
) -> u64 {
    println!("Cont Range: {:?}  workflow: {}", range, workflow_id);
    if workflow_id == "A" {
        return calc_size(&range);
    } else if workflow_id == "R" {
        return 0;
    }

    let mut total = 0;

    let rules = workflows.get(&workflow_id).unwrap();

    let mut rest_range = range;

    for rule in rules {
        if rule.default {
            total = total + get_workflow_count(rest_range, workflows, rule.next_id.to_owned());
            continue;
        }

        let mut range_ja = rest_range.to_owned();
        let mut range_na = rest_range.to_owned();
        let split_value = rule.value;

        match (rule.com, rule.categories) {
            (Compare::Bigger, PartType::X) => {
                range_ja.x.0 = range_ja.x.0.max(split_value + 1);
                range_na.x.1 = range_na.x.1.min(split_value);
            }
            (Compare::Bigger, PartType::S) => {
                range_ja.s.0 = range_ja.s.0.max(split_value + 1);
                range_na.s.1 = range_na.s.1.min(split_value);
            }
            (Compare::Bigger, PartType::M) => {
                range_ja.m.0 = range_ja.m.0.max(split_value + 1);
                range_na.m.1 = range_na.m.1.min(split_value);
            }
            (Compare::Bigger, PartType::A) => {
                range_ja.a.0 = range_ja.a.0.max(split_value + 1);
                range_na.a.1 = range_na.a.1.min(split_value);
            }
            (Compare::Smaller, PartType::X) => {
                range_ja.x.1 = range_ja.x.1.min(split_value - 1);
                range_na.x.0 = range_na.x.0.max(split_value);
            }
            (Compare::Smaller, PartType::S) => {
                range_ja.s.1 = range_ja.s.1.min(split_value - 1);
                range_na.s.0 = range_na.s.0.max(split_value);
            }
            (Compare::Smaller, PartType::M) => {
                range_ja.m.1 = range_ja.m.1.min(split_value - 1);
                range_na.m.0 = range_na.m.0.max(split_value);
            }
            (Compare::Smaller, PartType::A) => {
                range_ja.a.1 = range_ja.a.1.min(split_value - 1);
                range_na.a.0 = range_na.a.0.max(split_value);
            }
        };
        // println!(
        //     "Range: {:?} ja: {:?} na: {:?} comp:{:?} part: {:?} split: {} workflow: {}",
        //     rest_range, range_ja, range_na, rule.com, rule.categories,split_value, rule.next_id
        // );

        total = total + get_workflow_count(range_ja, workflows, rule.next_id.to_owned());
        rest_range = range_na;
    }

    total
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

fn parse_input(text: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let input = text.replace("\r", "");
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
                    let value: u32 = split_rules[0][2..].parse().unwrap();
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

    for workflow in rules_vec {
        hash_rules.insert(workflow.id, workflow.rules);
    }

    (hash_rules, parts)
}
