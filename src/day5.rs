fn get_seat_ids() -> Vec<usize> {
    crate::util::data_lines(5).map(|line| {
        line.chars().fold(0, |acc, c| {
            match c {
                'B' | 'R' => acc * 2 + 1,
                'F' | 'L' => acc * 2,
                _ => unreachable!(),
            }
        })
    }).collect()
}

pub fn part1() {
    println!("Day 5, Part 1: {}", get_seat_ids().into_iter().max().unwrap());
}

pub fn part2() {
    let seat_ids = get_seat_ids();
    let min = seat_ids.iter().min().unwrap();
    let max = seat_ids.iter().max().unwrap();
    let total: usize = seat_ids.iter().sum();
    let expected_total = (max - min + 1) * (max + min) / 2;
    println!("Day 5, Part 2: {}", expected_total - total);
}
