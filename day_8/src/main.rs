#[derive(Default, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}
impl Node {
    fn new(input: &Vec<usize>) -> Node {
        Node::parse(input).0
    }
    fn parse(input: &[usize]) -> (Node, usize) {
        let mut head = Node::default();
        let children_count = input[0];
        let metadata_len = input[1];
        let mut values_traversed = 2;
        for _ in 0..children_count {
            let (node, skip_amount) = Node::parse(&input[values_traversed..]);
            values_traversed += skip_amount;
            head.children.push(node);
        }
        head.metadata = input[values_traversed..metadata_len+values_traversed].to_vec();
        values_traversed += metadata_len;
        (head, values_traversed)
    }
    fn sum_with_children(&self) -> usize {
        let mut sum = self.metadata.iter().sum();
        for child in self.children.iter() {
           sum += child.sum_with_children();
        }
        sum
    }
    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let mut value = 0;
            for i in self.metadata.iter().filter(|&&i| i != 0) {
                if let Some(node) = self.children.get(i-1) {
                    value += node.value();
                }
            }
            value
        }
    }
}

fn part_1(input: &str) -> usize {
    let head = Node::new(&input.split_whitespace().map(|s| s.parse().unwrap()).collect());
    head.sum_with_children()
}

fn part_2(input: &str) -> usize {
    let head = Node::new(&input.split_whitespace().map(|s| s.parse().unwrap()).collect());
    head.value()
}

#[allow(dead_code)] // used in test functions
const TEST_INPUT: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 138);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 66);
}

fn main() {
    let (day, input) = trev_aoc_common::get_input();
    match day {
        Some(day) => match day {
            1 => println!("day 1: {}", part_1(input.as_str())),
            2 => println!("day 2: {}", part_2(input.as_str())),
            _ => {},
        },
        None => {
            println!("day 1: {}", part_1(input.as_str()));
            println!("day 2: {}", part_2(input.as_str()));
        }
    }
}
