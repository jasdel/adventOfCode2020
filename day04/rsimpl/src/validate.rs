use std::collections::HashMap;
use std::error::Error;

trait Validation {
    fn key(&self) -> &str;
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>>;
}

pub fn is_valid(record: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut required: Vec<&dyn Validation> = Vec::new();
    required.push(&NumberRange {
        key: "byr",
        min: 1920,
        max: 2002,
    });
    required.push(&NumberRange {
        key: "iyr",
        min: 2010,
        max: 2020,
    });
    required.push(&NumberRange {
        key: "eyr",
        min: 2020,
        max: 2030,
    });
    required.push(&Height {});
    required.push(&HairColor {});
    required.push(&EyeColor {});
    required.push(&PassportID {});

    for req in &required {
        let v = record.get(req.key());
        if v.is_none() {
            return Err(format!("not found: {}", req.key()).into());
        }
        let _ = req.is_valid(v.unwrap())?;
    }

    Ok(())
}

struct NumberRange {
    key: &'static str,
    min: u32,
    max: u32,
}

impl Validation for NumberRange {
    fn key(&self) -> &str {
        return self.key;
    }
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>> {
        let year: u32 = v.parse()?;
        if year < self.min || year > self.max {
            return Err(
                format!("{}, min:{}, max:{}, {}", self.key, self.min, self.max, year).into(),
            );
        }

        Ok(())
    }
}

struct Height {}
impl Validation for Height {
    fn key(&self) -> &str {
        "hgt"
    }
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>> {
        let sizes = [
            NumberRange {
                key: "cm",
                min: 150,
                max: 193,
            },
            NumberRange {
                key: "in",
                min: 59,
                max: 76,
            },
        ];

        for s in &sizes {
            if !v.ends_with(s.key()) {
                continue;
            }

            let v = v.strip_suffix(s.key()).unwrap();
            return s.is_valid(v);
        }

        Err(format!("hgt: format {}", v).into())
    }
}

struct HairColor {}
impl Validation for HairColor {
    fn key(&self) -> &str {
        "hcl"
    }
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>> {
        if v.len() != 7 {
            return Err(format!("hcl: length {}, {}", v.len(), v).into());
        }
        if !v.starts_with("#") {
            return Err(format!("hcl: format {}", v).into());
        }

        for c in v.get(1..).unwrap().chars() {
            match c {
                '0'..='9' => (),
                'a'..='f' => (),
                _ => return Err(format!("hcl: format {}, {}", c, v).into()),
            }
        }

        Ok(())
    }
}

struct EyeColor {}
impl Validation for EyeColor {
    fn key(&self) -> &str {
        "ecl"
    }
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>> {
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        for color in &colors {
            if *color == v {
                return Ok(());
            }
        }
        Err(format!("ecl: unknown {}", v).into())
    }
}

struct PassportID {}
impl Validation for PassportID {
    fn key(&self) -> &str {
        "pid"
    }
    fn is_valid(&self, v: &str) -> Result<(), Box<dyn Error>> {
        if v.len() != 9 {
            return Err(format!("pid: length {}, {}", v.len(), v).into());
        }

        for c in v.chars() {
            match c {
                '0'..='9' => (),
                _ => return Err(format!("pid: format {}, {}", c, v).into()),
            }
        }

        Ok(())
    }
}
