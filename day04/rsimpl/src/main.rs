use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::result::Result;

mod lex;
mod validate;

fn main() {
    let f = File::open("../input").expect("failed to open file");
    let mut f = BufReader::new(f);

    let c = part1(&mut f);
    match c {
        Ok(c) => println!("Part1: {}", c),
        Err(e) => println!("Part1 failed, {}", e),
    }

    f.seek(SeekFrom::Start(0)).expect("failed to seek");

    let c = part2(&mut f);
    match c {
        Ok(c) => println!("Part2: {}", c),
        Err(e) => println!("Part2 failed, {}", e),
    }
}

fn part1(f: &mut dyn BufRead) -> Result<u64, Box<dyn Error>> {
    let records = lex::tokenize(f)?;

    let required = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*, "cid"*/
    ];

    let mut found: u64 = 0;
    let mut missing = 0;
    'outer: for record in records {
        for req in &required {
            if !record.contains_key(*req) {
                println!("missing: {}, {:?}", *req, record);
                missing += 1;
                continue 'outer;
            }
        }
        found += 1;
        println!("found: {:?}", record);
    }

    println!("found: {}, missing: {}", found, missing);

    Ok(found)
}

fn part2(f: &mut BufReader<File>) -> Result<u64, Box<dyn Error>> {
    let records = lex::tokenize(f)?;

    let mut valid: u64 = 0;
    for record in records {
        match validate::is_valid(&record) {
            Ok(()) => {
                valid += 1;
                println!("valid: {:?}", record);
            }
            Err(e) => {
                println!("not valid: {}, {:?}", e, record);
            }
        }
    }

    Ok(valid)
}
