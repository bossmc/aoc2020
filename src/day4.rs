use std::collections::HashMap;

struct Passport(HashMap<String, String>);

fn load_data() -> Vec<Passport> {
    let mut res = Vec::new();
    let mut current = HashMap::new();
    for line in crate::util::data_lines(4) {
        if line.trim().is_empty() {
            res.push(Passport(current));
            current = HashMap::new();
        } else {
            current.extend(line.split(" ").map(|entry| {
                let (key, value) = <str as crate::util::StrExt>::split_once(entry, ":");
                (key.to_string(), value.to_string())
            }))
        }
    }
    res.push(Passport(current));
    res
}

macro_rules! o {
    ($exp:expr) => {
        match $exp {
            Some(exp) => exp,
            None => {
                return false;
            }
        }
    }
}

macro_rules! r {
    ($exp:expr) => {
        match $exp {
            Ok(exp) => exp,
            Err(_) => {
                return false;
            }
        }
    }
}

impl Passport {
    fn validate_old(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().all(|k| self.0.contains_key(*k))
    }

    fn validate_new(&self) -> bool {
        use std::str::FromStr as _;
        if !self.validate_old() {
            return false;
        }
        let byr = r!(usize::from_str(o!(self.0.get("byr"))));
        let iyr = r!(usize::from_str(o!(self.0.get("iyr"))));
        let eyr = r!(usize::from_str(o!(self.0.get("eyr"))));
        let hgt = o!(self.0.get("hgt"));
        let hcl = o!(self.0.get("hcl"));
        let ecl = o!(self.0.get("ecl"));
        let pid = o!(self.0.get("pid"));
        let byr_valid = (1920 <= byr) && (byr <= 2002);
        let iyr_valid = (2010 <= iyr) && (iyr <= 2020);
        let eyr_valid = (2020 <= eyr) && (eyr <= 2030);
        let hgt_valid = self.validate_hgt(hgt);
        let hcl_valid = (hcl.len() == 7) && (hcl.chars().nth(0).unwrap() == '#') && (hcl[1..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')));
        let ecl_valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|c| c == ecl);
        let pid_valid = (pid.len() == 9) && (pid.chars().all(|c| (c >= '0' && c <= '9')));
        byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
    }

    fn validate_hgt(&self, hgt: &str) -> bool {
        use std::str::FromStr as _;
        if hgt.ends_with("in") {
            let hgt = r!(usize::from_str(&hgt[..hgt.len() - 2]));
            (59 <= hgt) && (hgt <= 76)
        } else if hgt.ends_with("cm") {
            let hgt = r!(usize::from_str(&hgt[..hgt.len() - 2]));
            (150 <= hgt) && (hgt <= 193)
        } else {
            false
        }
    }
}

pub fn part1() {
    let data = load_data();
    println!("Day 4, Part 1: {}", data.iter().filter(|p| p.validate_old()).count());
}

pub fn part2() {
    let data = load_data();
    println!("Day 4, Part 2: {}", data.iter().filter(|p| p.validate_new()).count());
}
