use regex::Regex;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
struct Step {
    depends_on: Vec<char>,
    depending_on_this: Vec<char>,
    done: bool,
}

type Steps = HashMap<char, Step>;

fn parse_input(input: &String) -> Steps {
    let mut steps: HashMap<char, Step> = HashMap::new();
    let regex = Regex::new(r"Step (.) must be finished before step (.) can begin.")
        .expect("Failed to compile regex!");

    for captures in regex.captures_iter(input.as_str()) {
        let target = captures[2].chars().next().unwrap();
        let depend = captures[1].chars().next().unwrap();
        let step = steps.entry(target).or_default();
        step.depends_on.push(depend);
        let depend = steps.entry(depend).or_default();
        depend.depending_on_this.push(target);
    }
    steps
}

#[derive(Default)]
struct Available {
    steps: Steps,
    vector: Vec<char>,
}
impl Available {
    fn new(steps: Steps) -> Available {
        let mut ret = Available::default();
        ret.vector = steps
            .iter()
            .filter(|(_, steps)| steps.depends_on.is_empty())
            .map(|(id, _)| *id)
            .collect();
        ret.vector.sort();
        ret.steps = steps;
        ret
    }
    fn remove(&mut self, key: char) -> Vec<char> {
        self.steps.get_mut(&key).unwrap().done = true;
        self.vector
            .remove(self.vector.iter().position(|c| c == &key).unwrap());
        let new_values: Vec<char> = self
            .steps
            .get(&key)
            .unwrap()
            .depending_on_this
            .iter()
            .filter(|x| {
                self.steps
                    .get(x)
                    .unwrap()
                    .depends_on
                    .iter()
                    .all(|s| self.steps[s].done)
            })
            .map(|n| *n)
            .collect();
        self.vector.append(&mut new_values.clone());
        self.vector.sort();
        new_values
    }
    fn pop(&mut self) -> char {
        let current_id = self.vector[0];
        self.remove(current_id);
        current_id
    }
    fn is_done(&self) -> bool {
        self.vector.is_empty()
    }
}

fn day_1(steps: &Steps) -> String {
    let mut available = Available::new(steps.clone());
    let mut order = String::new();
    while !available.is_done() {
        order.push(available.pop());
    }
    order
}

#[derive(Debug, Copy, Clone)]
struct Job {
    id: char,
    finish_time: usize,
}

fn day_2(steps: &Steps) -> usize {
    let mut workers = [None; 5];
    let base_step_length = 60;
    let mut time = 0;
    let mut available = Available::new(steps.clone());
    let mut awaiting = available.vector.clone();
    while !available.is_done() {
        let mut workers_iter = workers.iter_mut().filter(|x| x.is_none());
        while let Some(worker) = workers_iter.next() {
            if let Some(id) = awaiting.pop() {
                *worker = Some(Job {
                    finish_time: time + base_step_length + (id as u8 - 'A' as u8) as usize + 1,
                    id,
                });
            }
        }
        let finished_worker_option = workers
            .iter_mut()
            .filter(|o| o.is_some())
            .min_by(|a, b| {
                a.as_ref()
                    .unwrap()
                    .finish_time
                    .cmp(&b.as_ref().unwrap().finish_time)
            })
            .unwrap();
        let finished_worker = finished_worker_option.take().unwrap();
        time = finished_worker.finish_time;
        awaiting.append(&mut available.remove(finished_worker.id));
    }
    time
}

fn main() {
    let (_, input) = trev_aoc_common::get_input();
    let steps = parse_input(&input);
    println!("Day 1 = {}", day_1(&steps));
    println!("Day 2 = {}", day_2(&steps));
}
