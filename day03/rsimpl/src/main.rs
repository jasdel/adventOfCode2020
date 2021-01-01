use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader};
use std::result::Result;

fn main() {
    let f = File::open("../input").expect("failed to open file");
    let mut f = BufReader::new(f);

    let c = part1(&mut f).expect("part1 failure");
    println!("Part1: {}", c);

    f.seek(SeekFrom::Start(0)).expect("failed to seek");

    let mut checks = [
        Check {
            x: 1,
            y: 1,
            x_offset: 0,
            y_offset: 0,
        },
        // base case
        Check {
            x: 3,
            y: 1,
            x_offset: 0,
            y_offset: 0,
        },
        Check {
            x: 5,
            y: 1,
            x_offset: 0,
            y_offset: 0,
        },
        Check {
            x: 7,
            y: 1,
            x_offset: 0,
            y_offset: 0,
        },
        Check {
            x: 1,
            y: 2,
            x_offset: 0,
            y_offset: 0,
        },
    ];
    let ns = part2(&mut f, &mut checks[..]);
    match ns {
        Ok(ns) => {
            let mut p = ns[0];
            for n in &ns[1..] {
                p *= n;
            }

            println!("Part2: {:?}, {}", ns, p);
        }
        Err(e) => println!("Part2 failed, {}", e),
    }
}

fn part1(f: &mut BufReader<File>) -> Result<u64, Box<dyn Error>> {
    let mut trees: u64 = 0;

    let mut offset: u64 = 0;
    let mut first: bool = true;
    for line in f.lines() {
        if first {
            first = false;
            continue;
        }
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let bytes = line.as_bytes();

        offset += 3;
        let o = offset % (bytes.len() as u64);
        if bytes[o as usize] == '#' as u8 {
            trees += 1;
        }
    }

    Ok(trees)
}

struct Check {
    x: u64,
    y: u64,
    x_offset: u64,
    y_offset: u64,
}

fn part2(f: &mut BufReader<File>, checks: &mut [Check]) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut ns: Vec<u64> = {
        let mut v = Vec::new();
        v.resize(checks.len(), 0);
        v
    };

    let mut first: bool = true;
    for line in f.lines() {
        if first {
            first = false;
            continue;
        }
        let line = line?;
        let bytes = line.as_bytes();

        for i in 0..checks.len() {
            let mut check = checks.get_mut(i).expect("whut?");
            check.y_offset += 1;
            if (check.y_offset % check.y) != 0 {
                continue;
            }

            check.x_offset += check.x;
            let o = check.x_offset % (bytes.len() as u64);
            if bytes[o as usize] == '#' as u8 {
                ns[i] += 1;
            }
        }
    }

    Ok(ns)
}
