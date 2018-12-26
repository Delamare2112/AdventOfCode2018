use regex::Regex;
use trev_aoc_common::run;

struct Point {
    x: isize,
    y: isize,
}

fn taxi_dist(a: &Point, b: &Point) -> usize {
    let p = Point {
        x: a.x - b.x,
        y: a.y - b.y,
    };
    (p.x.abs() + p.y.abs()) as usize
}

fn parse_input(input: String) -> Vec<Point> {
    let mut ret = Vec::new();

    let re = Regex::new(r"(\d+), (\d+)").expect("Failed to compile regex");
    for line in input.lines() {
        let cap = re.captures_iter(line).next().unwrap();
        ret.push(Point {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
        });
    }

    ret
}

fn part_1(input: String) -> usize {
    let input = parse_input(input);
    let mut counts = vec![0; input.len()];
    let bounds = Point {
        x: input.iter().map(|x| x.x).max().unwrap(),
        y: input.iter().map(|x| x.y).max().unwrap(),
    };
    let mut banned = Vec::new();

    for y in 0..bounds.y + 1 {
        for x in 0..bounds.x + 1 {
            let p1 = Point { x, y };
            let mut closest = usize::max_value();
            let mut closest_i = usize::max_value();
            let mut previous_closest = closest;
            for (i, p2) in input.iter().enumerate() {
                let distance = taxi_dist(&p1, &p2);
                if distance <= closest {
                    previous_closest = closest;
                    closest = distance;
                    closest_i = i;
                }
            }

            if closest == previous_closest {
                continue;
            }

            counts[closest_i] += 1;
            if y == 0 || y == bounds.y || x == 0 || x == bounds.x {
                banned.push(closest_i);
            }
        }
    }

    *counts
        .iter()
        .enumerate()
        .filter(|x| !banned.contains(&x.0))
        .map(|x| x.1)
        .max()
        .unwrap()
}

fn part_2(input: String) -> usize {
    let input = parse_input(input);
    let max_total_distance = 32;
    let bounds = Point {
        x: input.iter().map(|x| x.x).max().unwrap(),
        y: input.iter().map(|x| x.y).max().unwrap(),
    };
    let mut size = 0;
    for y in 0..bounds.y + 1 {
        for x in 0..bounds.x + 1 {
            let p = Point { x, y };
            let total_distance: usize = input.iter().map(|x| taxi_dist(x, &p)).sum();
            if total_distance < max_total_distance {
                size += 1;
            }
        }
    }
    size
}

fn main() {
    run(&part_1, &part_2);
}
