use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::result::Result;

fn main() {
    let f = File::open("../input").expect("failed to open file");
    let mut f = BufReader::new(f);

    match part1(&mut f, 128, 8) {
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

#[derive(Debug)]
struct Range(u32, u32);

fn part1(f: &mut BufReader<File>, rows: u32, cols: u32) -> Result<u32, Box<dyn Error>> {
    let mut id: u32 = 0;

    for line in f.lines() {
        let mut row = Range(0, rows - 1);
        let mut col = Range(0, cols - 1);

        let line = line?;
        for c in line.as_str().chars() {
            match c {
                'B' => {
                    // higher number
                    row.0 += (row.1 - row.0) / 2;
                    row.0 += 1;
                }
                'F' => {
                    // lower numbers
                    row.1 -= (row.1 - row.0) / 2;
                    row.1 -= 1;
                }
                'R' => {
                    // higher number
                    col.0 += (col.1 - col.0) / 2;
                    col.0 += 1;
                }
                'L' => {
                    // lower numbers
                    col.1 -= (col.1 - col.0) / 2;
                    col.1 -= 1;
                }
                _ => return Err(format!("unexpected {}", c).into()),
            }
        }

        println!("{}: {:?}, {:?}, {}", line, row, col, row.1 * 8 + col.1);
        let cur_id = row.1 * 8 + col.1;
        if cur_id > id {
            id = cur_id;
        }
    }

    Ok(id)
}

fn part2(f: &mut BufReader<File>) -> Result<u64, Box<dyn Error>> {
    Err("not implemented".into())
}
