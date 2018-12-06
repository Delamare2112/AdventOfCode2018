extern crate argparse;
extern crate chrono;
extern crate regex;

use argparse::{ArgumentParser, Store, StoreOption};
use chrono::prelude::*;
use regex::Regex;

use std::io::prelude::*;
use std::fs::File;
use std::collections::{BTreeMap, HashMap};

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

struct Day {
    guard: usize,
    times: [bool; 60]
}

fn events_to_grid(events: &BTreeMap<i64, Event>) -> Vec<Day> {
    let mut days = Vec::new();

    let first_event = events.iter().next().unwrap();
//    let mut current_date = Utc.timestamp(*first_event.0);
    let mut current_day = Day {guard: first_event.1.guard.unwrap(), times: [false; 60]};
    let mut sleep_start = 59;
    for (time, event) in events.iter().skip(1) {
        if event.t == EventType::StartShift {
            days.push(current_day);
            current_day = Day {guard: event.guard.unwrap(), times: [false; 60]};
        }
        else if event.t == EventType::FallAsleep {
            sleep_start = Utc.timestamp(*time, 0).minute();
        }
        else if event.t == EventType::Wakeup {
            for i in sleep_start..Utc.timestamp(*time, 0).minute() {
                current_day.times[i as usize] = true;
            }
        }
    }
    days.push(current_day);

    days
}

fn get_sleepiest_guard(grid: &Vec<Day>) -> usize {
    let mut guard_counts = HashMap::new();
    let mut sleepiest_guard = 0;
    let mut sleepiest_guard_time = 0;
    for day in grid.iter() {
        let guard = guard_counts.entry(day.guard).or_insert(0);
        *guard += day.times.iter().filter(|x| **x).count();
        if *guard > sleepiest_guard_time {
            sleepiest_guard = day.guard;
            sleepiest_guard_time = *guard;
        }
    }
    sleepiest_guard
}

fn output_days(days: &Vec<Day>) {
    print!("     ");
    for i in 0..6 {
        for _ in 0..10 {
            print!("{}", i);
        }
    }
    println!();
    print!("     ");
    for _ in 0..6 {
        for i in 0..10 {
            print!("{}", i);
        }
    }
    println!();
    for day in days.iter() {
        print!("{}   ", day.guard);
        for b in day.times.iter() {
            let val = if *b {'#'} else {'.'};
            print!("{}", val);
        }
        println!();
    }
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
    let events = parse_input(&input);
    let days = events_to_grid(&events);
    println!("{:?}", &events);
    output_days(&days);
    println!("sleepiest = {:?}", get_sleepiest_guard(&days));
}
