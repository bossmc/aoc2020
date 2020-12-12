fn load_data() -> Vec<Vec<bool>> {
    crate::util::data_lines(3).map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            }
        }).collect()
    }).collect()
}

fn perform_traverse(right: usize, down: usize, data: &[Vec<bool>]) -> usize {
    data.iter().step_by(down).enumerate().filter(|(row_id, row_data)| {
        row_data[(row_id * right) % row_data.len()]
    }).count()
}

pub fn part1() {
    let data = load_data();
    println!("Day 3, Part 1: {}", perform_traverse(3, 1, &data));
}

pub fn part2() {
    let data = load_data();
    let path1 = perform_traverse(1, 1, &data);
    let path2 = perform_traverse(3, 1, &data);
    let path3 = perform_traverse(5, 1, &data);
    let path4 = perform_traverse(7, 1, &data);
    let path5 = perform_traverse(1, 2, &data);
    println!("Day 3, Part 2: {}", path1 * path2 * path3 * path4 * path5);
}
