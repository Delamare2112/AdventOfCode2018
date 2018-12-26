use argparse::{ArgumentParser, Store, StoreOption};

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Display;

pub fn get_input() -> (Option<usize>, String) {
    let mut path = String::new();
    let mut part: Option<usize> = None;
    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut path)
            .add_argument("input", Store, "The input file");
        parser.refer(&mut part).add_option(
            &["--part"],
            StoreOption,
            "Which part to run. Both by default.",
        );
        parser.parse_args_or_exit();
    }

    let mut file = File::open(path).expect("File could not be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    (part, contents)
}

pub fn run<A: Display, B: Display>(part_1: &Fn(String) -> A, part_2: &Fn(String) -> B) {
    let (part, input) = get_input();

    if part.unwrap_or(1) == 1 {
        println!("part1: {}", part_1(input.clone()));
    }
    if part.unwrap_or(2) == 2 {
        println!("part2: {}", part_2(input.clone()));
    }
}
