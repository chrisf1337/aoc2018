use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const N_WORKERS: usize = 5;

lazy_static! {
    static ref INSTR_REGEX: Regex =
        Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    static ref COSTS: HashMap<char, u32> = {
        let mut costs = HashMap::new();
        let base_cost = 60;
        for ch in 'A' as u8..='Z' as u8 {
            costs.insert(ch as char, ch as u32 - 'A' as u32 + base_cost + 1);
        }
        costs
    };
}

#[derive(Debug, Clone)]
struct Node {
    name: char,
    parents: HashSet<char>,
    children: HashSet<char>,
}

impl Node {
    fn new(name: char) -> Node {
        Node {
            name,
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

fn build_dag(instructions: &[(char, char)]) -> (HashMap<char, Node>, HashSet<char>) {
    let mut nodes = HashMap::new();

    for &(from, to) in instructions {
        nodes.entry(from).or_insert(Node::new(from));
        nodes.entry(to).or_insert(Node::new(to));
        nodes.get_mut(&from).unwrap().children.insert(to);
        nodes.get_mut(&to).unwrap().parents.insert(from);
    }

    let roots = nodes
        .iter()
        .filter_map(|(&name, node)| {
            if node.parents.is_empty() {
                Some(name)
            } else {
                None
            }
        })
        .collect::<HashSet<char>>();

    (nodes, roots)
}

fn parse_instructions(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| {
            let caps = INSTR_REGEX.captures(l).unwrap();
            (
                caps[1].chars().next().unwrap(),
                caps[2].chars().next().unwrap(),
            )
        })
        .collect::<Vec<(char, char)>>()
}

pub fn part1(input: &str) -> String {
    let (nodes, mut roots) = build_dag(&parse_instructions(input));
    let mut order = vec![];
    while !roots.is_empty() {
        let mut possible_steps = vec![];
        for &root in &roots {
            let root_node = &nodes[&root];
            if root_node
                .parents
                .iter()
                .map(|p| order.contains(p))
                .all(|x| x)
            {
                possible_steps.push(root);
            }
        }
        possible_steps.sort_unstable();
        let step = possible_steps[0];
        roots.remove(&step);
        let step_node = &nodes[&step];
        roots.extend(&step_node.children);
        order.push(step);
    }

    order.into_iter().collect()
}

pub fn part2(input: &str) -> u32 {
    let (nodes, mut roots) = build_dag(&parse_instructions(input));
    let mut jobs: Vec<Option<(char, u32)>> = vec![None; N_WORKERS];
    let mut completed_tasks: HashSet<char> = HashSet::new();
    let mut n_ticks = 0;
    while completed_tasks.len() != nodes.len() {
        for job in jobs.iter_mut() {
            let mut done = false;
            if let Some((name, time_remaining)) = job {
                *time_remaining -= 1;
                if *time_remaining == 0 {
                    completed_tasks.insert(*name);
                    done = true;
                }
            }
            if done {
                *job = None;
            }
        }

        let mut possible_steps = vec![];
        for &root in &roots {
            let root_node = &nodes[&root];
            if root_node
                .parents
                .iter()
                .map(|p| completed_tasks.contains(p))
                .all(|x| x)
            {
                possible_steps.push(root);
            }
        }
        possible_steps.sort_unstable();

        for job in jobs.iter_mut() {
            if job.is_none() {
                if !possible_steps.is_empty() {
                    let step = possible_steps.remove(0);
                    *job = Some((step, COSTS[&step]));
                    let step_node = &nodes[&step];
                    roots.remove(&step);
                    roots.extend(&step_node.children);
                }
            }
        }

        n_ticks += 1;
    }

    n_ticks - 1
}
