use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::result::Result;

fn main() {
    let f = File::open("../input_test").expect("failed to open file");
    let mut f = BufReader::new(f);

    match part1(&mut f) {
        Ok(n) => println!("Part1: {}", n),
        Err(e) => println!("Part2 failed, {}", e),
    };

    f.seek(SeekFrom::Start(0)).expect("failed to seek");

    let ns = part2(&mut f);
    match ns {
        Ok(ns) => println!("Part2: {}", ns),
        Err(e) => println!("Part2 failed, {}", e),
    }
}

fn part1(f: &mut dyn BufRead) -> Result<u32, Box<dyn Error>> {
    Err("not implemented".into())
}

fn part2(f: &mut dyn BufRead) -> Result<u32, Box<dyn Error>> {
    Err("not implemented".into())
}
