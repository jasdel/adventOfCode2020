use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::result::Result;

fn main() {
    let f = File::open("../input").expect("failed to open file");
    let mut f = BufReader::new(f);

    let rows: u32 = 128;
    let cols: u32 = 8;

    match part1(&mut f, rows, cols) {
        Ok(n) => println!("Part1: {}", n),
        Err(e) => println!("Part2 failed, {}", e),
    };

    f.seek(SeekFrom::Start(0)).expect("failed to seek");

    let ns = part2(&mut f, rows, cols);
    match ns {
        Ok(ns) => println!("Part2: {}", ns),
        Err(e) => println!("Part2 failed, {}", e),
    }
}

#[derive(Debug)]
struct Range(u32, u32);

fn part1(f: &mut dyn BufRead, rows: u32, cols: u32) -> Result<u32, Box<dyn Error>> {
    let mut seat_id: u32 = 0;

    for line in f.lines() {
        let line = line?;

        let curr_seat_id = get_seat_id(line.as_str(), rows, cols)?;

        println!("{}: {}", line, curr_seat_id);
        if curr_seat_id > seat_id {
            seat_id = curr_seat_id;
        }
    }

    Ok(seat_id)
}

fn get_seat_id(line: &str, rows: u32, cols: u32) -> Result<u32, Box<dyn Error>> {
    let mut row = Range(0, rows - 1);
    let mut col = Range(0, cols - 1);

    for c in line.chars() {
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

    Ok(row.1 * cols + col.1)
}

fn part2(f: &mut dyn BufRead, rows: u32, cols: u32) -> Result<u32, Box<dyn Error>> {
    let mut seats = vec![false; (rows * cols) as usize];

    for line in f.lines() {
        let line = line?;

        let seat_id = get_seat_id(line.as_str(), rows, cols)?;
        seats[seat_id as usize] = true;
    }

    let mut starting = false;
    for (seat_id, found) in seats.iter().enumerate() {
        if !starting {
            if *found {
                starting = true;
            }
            continue;
        }

        let seat_id = seat_id as u32;
        if !found {
            return Ok(seat_id);
        }
    }

    Err("seat not found".into())
}
