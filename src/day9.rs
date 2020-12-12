fn load_data() -> Vec<usize> {
    crate::util::data_lines(9)
        .map(|line| std::str::FromStr::from_str(&line))
        .map(Result::unwrap)
        .collect()
}

pub fn part1() {
    for window in load_data().windows(26) {
        let (preamble, target) = (&window[..25], window[25]);
        if crate::util::find_pair(preamble, target).is_none() {
            println!("Day 9, Part 1: {}", target);
            break;
        }
    }
}

pub fn part2() {
    let mut low = 0;
    let mut high = 1;
    enum Parity {
        Wax,
        Wane,
    };
    let mut parity = Parity::Wax;
    let data = load_data();
    loop {
        let range = &data[low..=high];
        match (&parity, range.iter().sum::<usize>().cmp(&22406676)) {
            (Parity::Wax, std::cmp::Ordering::Less) => high += 1,
            (Parity::Wane, std::cmp::Ordering::Less) => {
                low += 1;
                parity = Parity::Wax;
            }
            (_, std::cmp::Ordering::Equal) => {
                println!(
                    "Day 9, Part 2: {}",
                    range.iter().min().unwrap() + range.iter().max().unwrap()
                );
                break;
            }
            (Parity::Wax, std::cmp::Ordering::Greater) => {
                low += 1;
                parity = Parity::Wane;
            }
            (Parity::Wane, std::cmp::Ordering::Greater) => {
                high -= 1;
            }
        }
    }
}
