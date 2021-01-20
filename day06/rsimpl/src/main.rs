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
    parse_groups_any(f)
}

fn part2(f: &mut dyn BufRead) -> Result<u32, Box<dyn Error>> {
    parse_groups_all(f)
}

fn parse_groups_any(f: &mut dyn BufRead) -> Result<u32, Box<dyn Error>> {
    let mut sum: u32 = 0;

    let mut group: HashSet<char> = HashSet::new();
    for line in f.lines() {
        let line = line?;
        if line.len() == 0 {
            sum += group.len() as u32;
            group.clear();
        }

        let answers = parse_answers(line.as_str())?;
        group.extend(answers);
    }

    if group.len() != 0 {
        sum += group.len() as u32;
    }

    Ok(sum)
}

fn parse_groups_all(f: &mut dyn BufRead) -> Result<u32, Box<dyn Error>> {
    let mut sum: u32 = 0;

    let mut group: HashSet<char> = HashSet::new();
    let mut in_group: bool = false;
    for line in f.lines() {
        let line = line?;

        if line.is_empty() {
            sum += group.len() as u32;
            group.clear();
            in_group = false;
            continue;
        }

        let answers = parse_answers(line.as_str())?;
        if !in_group {
            group.extend(answers);
            in_group = true;
            continue;
        }

        group = group.intersection(&answers).cloned().collect();
    }

    if in_group {
        sum += group.len() as u32;
    }

    Ok(sum)
}

fn parse_answers(line: &str) -> Result<HashSet<char>, Box<dyn Error>> {
    let mut answers = HashSet::new();

    for c in line.chars() {
        answers.insert(c);
    }

    Ok(answers)
}
