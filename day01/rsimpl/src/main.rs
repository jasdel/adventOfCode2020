use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1(2020);
    part2(2020);
}

fn part1(target: u64) {
    let f = File::open("../day_1_input").expect("failed to open file");
    let f = BufReader::new(f);

    let mut items = HashSet::new();

    for line in f.lines() {
        let line = line.expect("unable to read line");
        if line.is_empty() {
            continue;
        }

        let a: u64 = line.parse().expect("not a number");
        let b = target - a;

        if items.contains(&b) {
            println!("{}", a * b);
            return;
        }

        items.insert(a);
    }
}

fn part2(target: i64) {
    let f = File::open("../day_1_input").expect("failed to open file");
    let f = BufReader::new(f);

    let mut is = HashSet::new();
    let mut bcs = HashMap::new();

    for line in f.lines() {
        let line = line.expect("unable to read line");
        if line.is_empty() {
            continue;
        }

        let a: i64 = line.parse().expect("not a number");

        let bc = target - a;

        is.insert(a);
        bcs.insert(bc, a);
    }

    for (bc, a) in &bcs {
        for b in &is {
            if b == a {
                continue;
            }
            let c = bc - b;
            if c > 0 && is.contains(&c) && a + b + c == target {
                println!("{}, {}, {}, {}", a * b * c, a, b, c);
                return;
            }
        }
    }
}
