use std::collections::HashSet;

fn load_data() -> Vec<Vec<HashSet<char>>> {
    let mut res = Vec::new();
    let mut current = Vec::new();
    for line in crate::util::data_lines(6) {
        if line.trim().is_empty() {
            res.push(current);
            current = Vec::new();
        } else {
            let answers: HashSet<char> = line.chars().collect();
            current.push(answers);
        }
    }
    res.push(current);
    res
}

pub fn part1() {
    use itertools::Itertools as _;
    let data = load_data();
    println!("Day 6, Part 1: {}", data.into_iter().map(|group| {
        group.into_iter().tree_fold1(|a: HashSet<char>, b: HashSet<char>| -> HashSet<char> { a.union(&b).cloned().collect::<HashSet<_>>() }).unwrap().len()
    }).sum::<usize>());
}

pub fn part2() {
    use itertools::Itertools as _;
    let data = load_data();
    println!("Day 6, Part 2: {}", data.into_iter().map(|group| {
        group.into_iter().tree_fold1(|a: HashSet<char>, b: HashSet<char>| -> HashSet<char> { a.intersection(&b).cloned().collect::<HashSet<_>>() }).unwrap().len()
    }).sum::<usize>());
}
