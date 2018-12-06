extern crate argparse;
extern crate chrono;
extern crate regex;

use argparse::{ArgumentParser, Store, StoreOption};
use chrono::prelude::*;
use regex::Regex;

use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

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

#[derive(PartialEq, Debug)]
enum EventType {
    StartShift,
    FallAsleep,
    Wakeup
}

#[derive(Debug)]
struct Event {
    t: EventType,
    guard: Option<usize>
}

fn parse_input(input: &String) -> BTreeMap<i64, Event> {
    let mut collection = BTreeMap::new();

    let re = Regex::new(r"\[(.*)] (\w+) (\S+)").unwrap();
    for line in input.lines() {
        let cap = re.captures_iter(line).next().unwrap();
        let t = match cap[2].as_ref() {
            "falls" => EventType::FallAsleep,
            "wakes" => EventType::Wakeup,
            &_ => EventType::StartShift,
        };
        let mut guard = None;
        if t == EventType::StartShift {
            guard = Some(cap[3][1..].parse().unwrap());
        }
        let time = Utc.datetime_from_str(cap[1].as_ref(), "%Y-%m-%d %H:%M").unwrap().timestamp();
        collection.insert(time, Event {t, guard});
    }

    collection
}

fn main() {
    let (part, input) = get_input();
    println!("{:?}", parse_input(&input));
}
