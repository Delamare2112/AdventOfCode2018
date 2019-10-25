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
    fn sum(&self) -> usize {
        let mut sum = self.metadata.iter().sum();
        for child in self.children.iter() {
           sum += child.sum();
        }
        sum
    }
}

fn day_1(input: &str) -> usize {
    let head = Node::new(&input.split_whitespace().map(|s| s.parse().unwrap()).collect());
    head.sum()
}

#[test]
fn day_1_test() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
    assert_eq!(day_1(input.as_str()), 138);
}

fn main() {
    let (day, input) = trev_aoc_common::get_input();
    match day {
        Some(day) => match day {
            1 => println!("day 1: {}", day_1(input.as_str())),
//            2 => println!("day 2: {}", day_2(input.as_str())),
            _ => {},
        },
        None => {
            println!("day 1: {}", day_1(input.as_str()));
//            println!("day 2: {}", day_2(input.as_str()));
        }
    }
}
