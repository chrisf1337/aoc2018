fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<u32>,
    pub sum: Option<u32>,
}

struct Parser {
    pub pos: usize,
    pub input: Vec<u32>,
}

impl Parser {
    fn new(input: Vec<u32>) -> Parser {
        Parser { pos: 0, input }
    }

    fn read_num(&mut self) -> u32 {
        let n = self.input[self.pos];
        self.pos += 1;
        n
    }

    fn parse_node(&mut self) -> Node {
        let n_children = self.read_num();
        let n_metadata = self.read_num();
        let mut children = vec![];
        let mut metadata = vec![];
        for _ in 0..n_children {
            children.push(self.parse_node())
        }
        for _ in 0..n_metadata {
            metadata.push(self.read_num())
        }
        Node {
            children,
            metadata,
            sum: None,
        }
    }
}

fn sum_metadata(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() + node.children.iter().map(|n| sum_metadata(n)).sum::<u32>()
}

fn sum_value(node: &mut Node) -> u32 {
    if let Some(sum) = node.sum {
        sum
    } else {
        let sum = if node.children.is_empty() {
            node.metadata.iter().sum::<u32>()
        } else {
            let mut s = 0;
            for &m in &node.metadata {
                if m <= node.children.len() as u32 {
                    let child = &mut node.children[m as usize - 1];
                    s += sum_value(child);
                }
            }
            s
        };
        node.sum = Some(sum);
        sum
    }
}

pub fn part1(input: &str) -> u32 {
    sum_metadata(&Parser::new(parse_numbers(input)).parse_node())
}

pub fn part2(input: &str) -> u32 {
    sum_value(&mut Parser::new(parse_numbers(input)).parse_node())
}
