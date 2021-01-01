use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use std::result::Result;

pub fn tokenize(input: &mut dyn BufRead) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut records = Vec::new();
    let mut record = HashMap::new();

    for line in input.lines() {
        let line = line?;
        if line.len() == 0 {
            records.push(record);
            record = HashMap::new();
        }

        for part in line.split_whitespace() {
            if part.trim().len() == 0 {
                continue;
            }
            let i = part.find(':').expect("colon not found");
            // could use split_once, but not released yet
            let (k, v) = part.split_at(i);
            let mut v = v.trim().clone().to_string();
            v.drain(..1);

            if v.len() == 0 {
                continue;
            }

            record.insert(k.trim().clone().to_string(), v);
        }
    }

    if record.len() != 0 {
        records.push(record);
    }

    Ok(records)
}
