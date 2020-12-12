fn load_data() -> impl Iterator<Item = u128> {
    crate::util::data_lines(10)
        .map(|line| std::str::FromStr::from_str(&line))
        .map(Result::unwrap)
}

pub fn part1() {
    let mut data = load_data().collect::<Vec<_>>();
    data.sort_unstable();
    let mut counts = [0, 0, 1]; // There's always a 3-gap between device and highest converter
    counts[(data[0] - 1) as usize] += 1;
    for window in data.windows(2) {
        counts[(window[1] - window[0] - 1) as usize] += 1;
    }
    println!("Day 10, Part 1: {}", counts[0] * counts[2]);
}

pub fn part2() {
    let mut max = 0;
    let data = load_data().map(|v| { if v > max { max = v }; v }).collect::<std::collections::HashSet::<_>>();
    let mut back_one = 1u128;
    let mut back_two = 0u128;
    let mut back_three = 0u128;
    let mut back_none = 0u128;
    for i in 1..=max {
        back_none = if data.contains(&i) {
            back_one + back_two + back_three
        } else {
            0
        };
        back_three = back_two;
        back_two = back_one;
        back_one = back_none;
    }
    println!("Day 10, Part 2: {}", back_none);
}
