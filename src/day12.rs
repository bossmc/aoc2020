#[derive(Debug, Copy, Clone)]
enum Instruction {
    Forward(isize),
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Left,
    Right,
    UTurn,
}

#[derive(Debug)]
struct Ship {
    y: isize,
    x: isize,
    heading: Heading,
}

impl Ship {
    fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            heading: Heading::East,
        }
    }

    fn sail(&mut self, y: isize, x: isize) {
        self.y += y;
        self.x += x;
    }

    fn step(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward(arg) => match self.heading {
                Heading::North => self.step(Instruction::North(arg)),
                Heading::East => self.step(Instruction::East(arg)),
                Heading::South => self.step(Instruction::South(arg)),
                Heading::West => self.step(Instruction::West(arg)),
            },
            Instruction::North(arg) => self.sail(arg, 0),
            Instruction::East(arg) => self.sail(0, arg),
            Instruction::South(arg) => self.sail(-arg, 0),
            Instruction::West(arg) => self.sail(0, -arg),
            Instruction::Left => self.heading.left(),
            Instruction::Right => self.heading.right(),
            Instruction::UTurn => self.heading.uturn(),
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Ship2 {
    y: isize,
    x: isize,
    wp_y: isize,
    wp_x: isize,
}

impl Ship2 {
    fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            wp_y: 1,
            wp_x: 10,
        }
    }

    fn step(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward(arg) => {
                self.y += self.wp_y * arg;
                self.x += self.wp_x * arg;
            }
            Instruction::North(arg) => self.wp_y += arg,
            Instruction::East(arg) => self.wp_x += arg,
            Instruction::South(arg) => self.wp_y -= arg,
            Instruction::West(arg) => self.wp_x -= arg,
            Instruction::Left => {
                std::mem::swap(&mut self.wp_y, &mut self.wp_x);
                self.wp_x *= -1;
            }
            Instruction::Right => {
                std::mem::swap(&mut self.wp_y, &mut self.wp_x);
                self.wp_y *= -1;
            }
            Instruction::UTurn => {
                self.wp_y *= -1;
                self.wp_x *= -1;
            }
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn right(&mut self) {
        *self = match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn left(&mut self) {
        *self = match self {
            Heading::North => Heading::West,
            Heading::East => Heading::North,
            Heading::South => Heading::East,
            Heading::West => Heading::South,
        }
    }

    fn uturn(&mut self) {
        *self = match self {
            Heading::North => Heading::South,
            Heading::East => Heading::West,
            Heading::South => Heading::North,
            Heading::West => Heading::East,
        }
    }
}

fn load_data() -> impl Iterator<Item = Instruction> {
    crate::util::data_lines(12).map(|line| {
        let arg = <isize as std::str::FromStr>::from_str(&line[1..]).unwrap();
        match line.as_bytes()[0] {
            b'F' => Instruction::Forward(arg),
            b'N' => Instruction::North(arg),
            b'E' => Instruction::East(arg),
            b'S' => Instruction::South(arg),
            b'W' => Instruction::West(arg),
            b'R' => match arg {
                90 => Instruction::Right,
                180 => Instruction::UTurn,
                270 => Instruction::Left,
                _ => unreachable!("Non-ortholinear turn"),
            }
            b'L' => match arg {
                90 => Instruction::Left,
                180 => Instruction::UTurn,
                270 => Instruction::Right,
                _ => unreachable!("Non-ortholinear turn"),
            }
            _ => unreachable!(),
        }
    })
}

pub fn part1() {
    let data = load_data();
    let mut ship = Ship::new();
    for inst in data {
        ship.step(inst);
    }
    println!("Day 12, Part 1: {}", ship.manhattan_distance());
}

pub fn part2() {
    let data = load_data().collect::<Vec<_>>();
    let mut ship = Ship2::new();
    for inst in data {
        ship.step(inst);
    }
    println!("Day 12, Part 2: {}", ship.manhattan_distance());
}
