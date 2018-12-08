use argparse::{ArgumentParser, Store, StoreOption};

use std::fs::File;
use std::io::prelude::*;

fn get_input() -> (Option<usize>, String) {
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

fn part_1(polymer: String) -> usize {
    let mut i = 0;
    let mut polymer: Vec<char> = polymer.chars().collect();
    while i < polymer.len() - 1 {
        loop {
            if (polymer[i].is_lowercase() ^ polymer[i + 1].is_lowercase())
                && polymer[i].to_lowercase().to_string()
                    == polymer[i + 1].to_lowercase().to_string()
            {
                polymer.remove(i);
                polymer.remove(i);
                if i != 0 {
                    i -= 1;
                }
                if i == polymer.len() - 1 {
                    break;
                }
            } else {
                break;
            }
        }
        i += 1;
    }
    polymer.len()
}

fn main() {
    let (part, input) = get_input();

    if part.unwrap_or(1) == 1 {
        println!("part1: {}", part_1(input.clone()));
    }
}
