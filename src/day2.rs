#[derive(recap::Recap, serde::Deserialize, Debug)]
#[recap(regex=r#"^(?P<low>\d+)-(?P<high>\d+) (?P<letter>[a-z]): (?P<data>[a-z]+)$"#)]
struct PasswordEntry {
    letter: char,
    low: usize,
    high: usize,
    data: String,
}

impl PasswordEntry {
    fn validate_old(&self) -> bool {
        let occurrences = self.data.chars().filter(|c| *c == self.letter).count();
        (self.low <= occurrences) && (occurrences <= self.high)
    }

    fn validate_new(&self) -> bool {
        let first = self.data.chars().nth(self.low - 1).unwrap();
        let second = self.data.chars().nth(self.high - 1).unwrap();
        (first == self.letter) ^ (second == self.letter)
    }
}

fn load_data() -> Vec<PasswordEntry> {
    crate::util::data_lines(2).map(|s| s.parse::<PasswordEntry>().unwrap()).collect()
}

pub fn part1() {
    let data = load_data();
    let valid = data.iter().filter(|p| p.validate_old());
    println!("Day 2, Part 1: {}", valid.count());
}

pub fn part2() {
    let data = load_data();
    let valid = data.iter().filter(|p| p.validate_new());
    println!("Day 2, Part 2: {}", valid.count());
}

#[test]
fn test_1_3_a_abcde() {
    let p = "1-3 a: abcde".parse::<PasswordEntry>().unwrap();
    assert!(p.validate_new());
    assert!(p.validate_old());
}

#[test]
fn test_1_3_b_cdefg() {
    let p = "1-3 a: cdefg".parse::<PasswordEntry>().unwrap();
    assert!(!p.validate_new());
    assert!(!p.validate_old());
}

#[test]
fn test_2_9_c_ccccccccc() {
    let p = "2-9 c: ccccccccc".parse::<PasswordEntry>().unwrap();
    assert!(!p.validate_new());
    assert!(p.validate_old());
}
