extern crate argparse;
extern crate regex;

use argparse::{ArgumentParser, Store, StoreOption};
use regex::Regex;

use std::io::prelude::*;
use std::fs::File;

fn get_input() -> (Option<usize>, String) {
    let mut path = String::new();
    let mut part: Option<usize> = None;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut path).add_argument("input", Store, "The input file");
        parser.refer(&mut part).add_option(&["--part"], StoreOption, "Which part to run. Both by default.");
        parser.parse_args_or_exit();
    }

    let mut file = File::open(path).expect("File could not be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");

    (part, contents)
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

fn input_to_claims(input: &String) -> Vec<Claim> {
    let mut ret = Vec::new();

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in input.lines() {
        let cap = re.captures_iter(line).next().unwrap();
        ret.push(Claim {
            id: cap[1].parse().unwrap(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            w: cap[4].parse().unwrap(),
            h: cap[5].parse().unwrap()
        });
    }

    ret
}

fn part_1(claims: &Vec<Claim>) -> usize {
    let mut overlaps = 0;
    let mut grid = vec![vec![Vec::new(); 10000]; 10000];

    for claim in claims.iter() {
        for y in grid.iter_mut().skip(claim.y).take(claim.h) {
            for x in y.iter_mut().skip(claim.x).take(claim.w) {
                if x.len() == 1 {
                    overlaps += 1;
                }
                x.push(claim.id);
            }
        }
    }

    overlaps
}

fn part_2(claims: &Vec<Claim>) -> usize {
    0
}

fn main() {
    let (part, input) = get_input();

    let claims = input_to_claims(&input);

    match part {
        Some(p) => {
            if p == 1 { println!("Part 1 = {}", part_1(&claims))}
            else if p == 2 { println!("Part 2 = {}", part_2(&claims))}
        }
        None => {
            println!("Part 1 = {}", part_1(&claims));
            println!("Part 2 = {}", part_2(&claims));
        }
    }

}
