#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Floor,
    Empty,
    Full,
}

#[derive(Debug)]
struct Direction(i8, i8);

const DIRECTIONS: &[Direction] = &[
    Direction(-1, -1),
    Direction(-1, 0),
    Direction(-1, 1),
    Direction(0, -1),
    Direction(0, 1),
    Direction(1, -1),
    Direction(1, 0),
    Direction(1, 1),
];

enum Mode {
    Immediate,
    Distant,
}

struct Board {
    data: Vec<Vec<State>>,
}

impl Board {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn get_cell(&self, y: usize, x: usize) -> State {
        self.data[y][x]
    }

    fn apply_offset(&self, y: usize, x: usize, direction: &Direction) -> Option<(usize, usize)> {
        let y = y as isize + direction.0 as isize;
        let x = x as isize + direction.1 as isize;
        if x < 0 || y < 0 {
            None
        } else {
            let y = y as usize;
            let x = x as usize;
            if x >= self.width() || y >= self.height() {
                None
            } else {
                Some((y, x))
            }
        }
    }

    fn get_immediate_neighbours(&self, y: usize, x: usize) -> impl Iterator<Item = State> + '_ {
        DIRECTIONS
            .iter()
            .filter_map(move |direction| self.apply_offset(y, x, direction))
            .map(move |(y, x)| self.data[y][x])
    }

    fn get_distant_neighbours(&self, y: usize, x: usize) -> impl Iterator<Item = State> + '_ {
        DIRECTIONS.iter().filter_map(move |direction| {
            let mut pos = (y, x);
            loop {
                pos = match self.apply_offset(pos.0, pos.1, direction) {
                    Some(pos) => pos,
                    None => return None,
                };
                if self.data[pos.0][pos.1] != State::Floor {
                    return Some(self.data[pos.0][pos.1]);
                }
            }
        })
    }

    fn step(&mut self, mode: Mode, vacate_limit: usize) -> bool {
        let mut new = self.data.clone();
        let mut changed = false;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let old_cell = self.get_cell(y, x);
                if old_cell != State::Floor {
                    let neighs = match mode {
                        Mode::Immediate => self
                            .get_immediate_neighbours(y, x)
                            .filter(|state| *state == State::Full)
                            .count(),
                        Mode::Distant => self
                            .get_distant_neighbours(y, x)
                            .filter(|state| *state == State::Full)
                            .count(),
                    };
                    if neighs == 0 {
                        new[y][x] = State::Full;
                        if old_cell != State::Full {
                            changed = true;
                        }
                    } else if neighs >= vacate_limit {
                        new[y][x] = State::Empty;
                        if old_cell != State::Empty {
                            changed = true;
                        }
                    }
                }
            }
        }
        self.data = new;
        changed
    }

    fn count_people(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|state| **state == State::Full).count())
            .sum()
    }
}

fn load_data() -> Board {
    Board {
        data: crate::util::data_lines(11)
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'L' => State::Empty,
                        b'.' => State::Floor,
                        b'#' => State::Full,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}

pub fn part1() {
    let mut data = load_data();
    while data.step(Mode::Immediate, 4) {}
    println!("Day 11, Part 1: {}", data.count_people());
}

pub fn part2() {
    let mut data = load_data();
    while data.step(Mode::Distant, 5) {}
    println!("Day 11, Part 2: {}", data.count_people());
}
