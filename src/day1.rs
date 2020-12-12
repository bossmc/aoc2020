fn load_data() -> Vec<usize> {
    crate::util::data_lines(1).map(|s| <usize as std::str::FromStr>::from_str(&s)).map(Result::unwrap).collect()
}

pub fn part1() {
    let data = load_data();
    let (a, b) = crate::util::find_pair(&data, 2020).unwrap();
    println!("Day 1, Part 1: {}", a * b);
}

pub fn part2() {
    let data = load_data();
    for idx in 0..data.len() - 2 {
        let first = data[idx];
        if let Some((second, third)) = crate::util::find_pair(&data[idx + 1..], 2020 - first) {
            println!("Day 1, Part 2: {}", first * second * third);
            return;
        }
    }
    panic!("No result found");
}
