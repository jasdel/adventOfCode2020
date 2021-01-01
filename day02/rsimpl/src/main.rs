use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

fn main() {
    let f = File::open("../input").expect("failed to open file");
    let mut f = BufReader::new(f);

    let c = part1(&mut f).expect("failure");
    println!("Part1: {}", c);

    f.seek(SeekFrom::Start(0)).expect("failed to seek");
    let c = part2(&mut f).expect("failure");

    println!("Part2: {}", c);
}

fn part1(f: &mut BufReader<File>) -> Option<u64> {
    let mut c: u64 = 0;

    for line in f.lines() {
        let line = line.expect("unable to read line");
        if line.is_empty() {
            continue;
        }

        let (min, max, ch, pass) =
            scan_fmt!(line.as_str(), "{d}-{d} {}: {}", u64, u64, char, String).expect("no match");

        let mut cs: u64 = 0;
        for pc in pass.chars() {
            if pc == ch {
                cs += 1
            }
        }
        if cs >= min && cs <= max {
            c += 1
        }
    }

    Some(c)
}

fn part2(f: &mut BufReader<File>) -> Option<u64> {
    let mut c: u64 = 0;

    for line in f.lines() {
        let line = line.expect("unable to read line");
        if line.is_empty() {
            continue;
        }

        let (a, b, ch, pass) =
            scan_fmt!(line.as_str(), "{d}-{d} {}: {}", u64, u64, char, String).expect("no match");

        let mut have_a: bool = false;
        let mut have_b: bool = false;
        for (i, pc) in pass.char_indices() {
            if (i + 1) as u64 == a && pc == ch {
                have_a = true;
            }
            if (i + 1) as u64 == b && pc == ch {
                have_b = true;
            }
        }

        if have_a ^ have_b {
            c += 1;
        }
    }

    Some(c)
}
