fn load_data() -> (usize, std::collections::HashMap<usize, usize>) {
    let mut data = std::collections::HashMap::new();
    data.insert(0, 1);
    data.insert(1, 2);
    data.insert(4, 3);
    data.insert(13, 4);
    data.insert(15, 5);
    data.insert(12, 6);
    data.insert(16, 7);
    (0, data)
}

fn play_the_game(turns: usize) -> usize {
    let (mut next, mut seen) = load_data();
    for turn in 8..turns {
        let tmp_next = match seen.get(&next) {
            Some(prev_turn) => turn - prev_turn,
            None => 0,
        };
        seen.insert(next, turn);
        next = tmp_next;
    }
    next
}

pub fn part1() {
    println!("Day 15, Part 1: {}", play_the_game(2020));
}

pub fn part2() {
    println!("Day 15, Part 2: {}", play_the_game(30_000_000));
}
