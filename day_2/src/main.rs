extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn get_input() -> String {
    let mut path = String::new();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Box scanning day!");
        parser.refer(&mut path).add_argument("input", Store, "ITS THE INPUT FILE PATH!!!");
        parser.parse_args_or_exit();
    }

    let mut file = File::open(path).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read input file");

    contents
}

fn part_1(input: &String) -> usize {
    let mut two = 0;
    let mut three = 0;

    for line in input.lines() {
        let mut collection = HashMap::new();
        for c in line.chars() {
            let entry = collection.entry(c).or_insert(0);
            *entry += 1;
        }

        if collection.values().any(|val| *val == 2) {
            two += 1;
        }
        if collection.values().any(|val| *val == 3) {
            three += 1;
        }
    }

    two * three
}

fn score(x: &str, y: &str) -> usize {
    let mut ret = 0;
    for (i, c) in x.chars().enumerate() {
        if y.chars().nth(i).unwrap() == c {
            ret += 1;
        }
    }
    ret
}

fn drop_diff(x: &str, y: &str) -> String {
    let mut ret = String::new();
    for (i, c) in x.chars().enumerate() {
        if y.chars().nth(i).unwrap() == c {
            ret.push(c);
        }
    }
    ret
}

fn part_2(input: &String) -> String {
    let target_score = input.lines().next().unwrap().len() - 1;
    for (s_i, s_l) in input.lines().enumerate() {
        for line in input.lines().skip(s_i) {
            if score(s_l, line) == target_score {
                return drop_diff(s_l, line);
            }
        }
    }
    String::new()
}

fn main() {
    let input = get_input();

    println!("Part 1 = {}", part_1(&input));
    println!("Part 2 = {}", part_2(&input));
}
