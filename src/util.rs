pub fn data_lines(day: usize) -> impl Iterator<Item = String> {
    use std::io::BufRead as _;
    let filename = format!("data/day{}.data", day);
    let file = std::fs::File::open(filename).unwrap();
    let file = std::io::BufReader::new(file);
    file.lines().map(Result::unwrap)
}

pub trait StrExt {
    fn split_once(&self, substring: &str) -> (&Self, &Self);
}

impl StrExt for str {
    fn split_once(&self, substring: &str) -> (&Self, &Self) {
        let idx = self.find(substring).unwrap();
        (&self[..idx], &self[idx + substring.len()..])
    }
}

pub fn find_pair<T: Copy + Ord + PartialEq + std::ops::Add<T, Output = T>>(
    data: &[T],
    target: T,
) -> Option<(T, T)> {
    let mut data = data.to_vec();
    data.sort_unstable();
    let mut low = 0;
    let mut high = data.len() - 1;
    while low < high {
        if data[low] + data[high] == target {
            return Some((data[low], data[high]));
        } else if data[low] + data[high] > target {
            high -= 1;
        } else {
            low += 1;
        }
    }
    None
}
