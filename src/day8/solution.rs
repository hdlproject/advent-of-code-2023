use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    commands: Vec<char>,
    nodes: HashMap<String, Node>,
}

pub struct Node {
    left: String,
    right: String,
    dead_end: bool,
}

impl Day {
    pub fn new(input_file: &str) -> Day {
        let file = File::open(input_file.trim()).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let commands: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
        let mut nodes = HashMap::new();
        for line_result in lines {
            let line = line_result.unwrap();
            if line == "".to_string() {
                continue;
            }

            let first_split = line
                .split(" = (")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>();
            let second_split = first_split[1]
                .split(", ")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>();

            let head = first_split[0].to_string();
            let left = second_split[0].to_string();
            let right = second_split[1]
                .to_string()
                .strip_suffix(")")
                .unwrap()
                .to_string();
            let dead_end = (head.as_str() == left.as_str()) && (head.as_str() == right.as_str());
            nodes.insert(
                head,
                Node {
                    left,
                    right,
                    dead_end,
                },
            );
        }

        Day { commands, nodes }
    }

    pub fn solve(&self) -> u64 {
        let mut step: u64 = Default::default();

        let mut current_index_command = 0;
        let mut current_node: String = "AAA".to_string();
        let finish_node = "ZZZ".to_string();
        loop {
            if self.nodes.contains_key(current_node.as_str()) {
                if self.nodes.get(current_node.as_str()).unwrap().dead_end == true {
                    break;
                }

                current_node = match self.commands[current_index_command] {
                    'R' => self.nodes.get(&current_node).unwrap().right.clone(),
                    'L' => self.nodes.get(&current_node).unwrap().left.clone(),
                    _ => todo!(),
                };
                step += 1;

                if current_node == finish_node {
                    break;
                } else {
                    current_index_command = (current_index_command + 1) % (self.commands.len());
                }
            } else {
                break;
            }
        }

        step
    }

    pub fn solve2_brute_force(&self) -> u64 {
        let mut step: u64 = Default::default();

        let mut current_index_command = 0;

        let mut current_nodes: Vec<String> = Default::default();
        let mut finish_nodes: Vec<String> = Default::default();
        for (head, node) in &self.nodes {
            if head.ends_with("A") {
                current_nodes.push(head.clone())
            }
            if head.ends_with("Z") {
                finish_nodes.push(head.clone())
            }
        }

        loop {
            let mut finish_count: u32 = Default::default();
            for current_node in current_nodes.iter_mut() {
                if self.nodes.contains_key(current_node.as_str()) {
                    if self.nodes.get(current_node.as_str()).unwrap().dead_end == true {
                        break;
                    }

                    *current_node = match self.commands[current_index_command] {
                        'R' => self.nodes.get(current_node).unwrap().right.clone(),
                        'L' => self.nodes.get(current_node).unwrap().left.clone(),
                        _ => todo!(),
                    };

                    if finish_nodes.contains(current_node) {
                        finish_count += 1;
                    }
                } else {
                    break;
                }
            }

            step += 1;

            if finish_count == current_nodes.len() as u32 {
                break;
            } else {
                current_index_command = (current_index_command + 1) % (self.commands.len());
            }
        }

        step
    }

    pub fn solve2(&self) -> u64 {
        let mut step: u64 = Default::default();

        let mut current_index_command = 0;

        let mut start_node_current_nodes: HashMap<String, String> = Default::default();
        let mut start_node_finish_step: HashMap<String, u64> = Default::default();
        let mut finish_nodes: Vec<String> = Default::default();
        for (head, node) in &self.nodes {
            if head.ends_with("A") {
                start_node_current_nodes.insert(head.clone(), head.clone());
                start_node_finish_step.insert(head.clone(), 0);
            }
            if head.ends_with("Z") {
                finish_nodes.push(head.clone())
            }
        }

        let mut finish_count: u32 = Default::default();
        loop {
            for (start_node, current_node) in start_node_current_nodes.iter_mut() {
                if current_node == "FINISH" {
                    continue;
                }

                if self.nodes.contains_key(current_node.as_str()) {
                    if self.nodes.get(current_node.as_str()).unwrap().dead_end == true {
                        break;
                    }

                    *current_node = match self.commands[current_index_command] {
                        'R' => self.nodes.get(current_node).unwrap().right.clone(),
                        'L' => self.nodes.get(current_node).unwrap().left.clone(),
                        _ => todo!(),
                    };

                    let finish_step = start_node_finish_step.get_mut(start_node).unwrap();
                    *finish_step += 1;
                    if finish_nodes.contains(current_node) {
                        finish_count += 1;
                        *current_node = "FINISH".to_string();
                    }
                } else {
                    break;
                }
            }

            if finish_count == start_node_current_nodes.len() as u32 {
                break;
            } else {
                current_index_command = (current_index_command + 1) % (self.commands.len());
            }
        }

        for (_, finish_step) in start_node_finish_step.iter() {
            if step == 0 {
                step = *finish_step;
                continue;
            }

            step = self.lcm(step, *finish_step);
        }

        step
    }

    fn gcd(&self, mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn lcm(&self, a: u64, b: u64) -> u64 {
        (a * b) / self.gcd(a, b)
    }
}
