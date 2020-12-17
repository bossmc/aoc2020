use std::collections::HashMap;

#[derive(Debug)]
struct Automaton<C: Coordinate> {
    cells: HashMap<C, Cell>,
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Active,
    Inactive,
}

trait Coordinate: std::hash::Hash + PartialEq + Eq + Clone + Copy {
    /// Expand a 2D coordinate to this coordinate system
    fn from_x_y(x: isize, y: isize) -> Self;
    /// I can't be bothered to `impl Add<Self> for (A, B, C)` due to orphan rules getting in the
    /// way, so here's an associated function that implements `+` with less sugar.
    ///
    /// "No thanks Turkish, I'm sweet enough already" -- Brick Top
    fn add_offset(&self, other: &Self) -> Self;
    /// The list of offsets to "neighbouring" coordinates
    fn offsets() -> Box<dyn Iterator<Item = Self>>;
}

impl Coordinate for (isize, isize, isize) {
    fn from_x_y(x: isize, y: isize) -> Self {
        (x, y, 0)
    }

    fn add_offset(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn offsets() -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            THREE_CUBE_COORDS
                .iter()
                .copied()
                // Remember not to include "no offset"
                .filter(|c| *c != (0, 0, 0)),
        )
    }
}

impl Coordinate for (isize, isize, isize, isize) {
    fn from_x_y(x: isize, y: isize) -> Self {
        (x, y, 0, 0)
    }

    fn add_offset(&self, other: &Self) -> Self {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }

    fn offsets() -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            [-1, 0, 1]
                .iter()
                .flat_map(|q| {
                    THREE_CUBE_COORDS
                        .iter()
                        .map(move |(x, y, z)| (*x, *y, *z, *q))
                })
                // Remember not to include "no offset"
                .filter(|o| *o != (0, 0, 0, 0)),
        )
    }
}

impl<C: Coordinate> Automaton<C> {
    fn load_from_data() -> Self {
        Self {
            cells: crate::util::data_lines(17)
                .enumerate()
                .flat_map(move |(y, line)| {
                    line.into_bytes()
                        .into_iter()
                        .enumerate()
                        .map(move |(x, byte)| {
                            (
                                C::from_x_y(x as isize, y as isize),
                                if byte == b'#' {
                                    Cell::Active
                                } else {
                                    Cell::Inactive
                                },
                            )
                        })
                })
                .collect(),
        }
    }

    /// Given a coordinate, calculate the state of that cell after one step
    fn update_cell(&self, coord: C) -> Cell {
        let active_neighbours = C::offsets()
            .filter(|offset| {
                matches!(
                    self.cells.get(&coord.add_offset(offset)),
                    Some(Cell::Active)
                )
            })
            .count();
        match self.cells.get(&coord) {
            Some(Cell::Active) if active_neighbours == 2 || active_neighbours == 3 => Cell::Active,
            Some(Cell::Active) => Cell::Inactive,
            _ if active_neighbours == 3 => Cell::Active,
            _ => Cell::Inactive,
        }
    }

    /// Copies out the old grid, dropping unreachable cells (`Inactive` without any `Active`
    /// neighbours) and adding newly reachable cells (all neighbours of `Active` cells)
    fn duplicate_grid(&self) -> HashMap<C, Cell> {
        let mut new = HashMap::new();
        for (coord, cell) in &self.cells {
            if *cell == Cell::Active {
                new.insert(*coord, Cell::Active);
                for offset in C::offsets() {
                    new.entry(coord.add_offset(&offset))
                        .or_insert(Cell::Inactive);
                }
            }
        }
        new
    }

    /// Drive the automaton forward one step
    fn step(&mut self) {
        let mut new = self.duplicate_grid();
        for (coord, ref mut v) in new.iter_mut() {
            **v = self.update_cell(*coord);
        }
        self.cells = new;
    }

    /// Count the number of active cells
    fn count_active(&self) -> usize {
        self.cells.values().filter(|v| **v == Cell::Active).count()
    }
}

/// The lattice defined by the intersection of Z^3 with the cube centered at the origin with edges
/// of length 2, oriented along the axis.
const THREE_CUBE_COORDS: &[(isize, isize, isize)] = &[
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 0),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

pub fn part1() {
    let mut automaton = Automaton::<(_, _, _)>::load_from_data();

    for _ in 0..6 {
        automaton.step();
    }

    println!("Day 17, Part 1: {}", automaton.count_active());
}

pub fn part2() {
    let mut automaton = Automaton::<(_, _, _, _)>::load_from_data();

    for _ in 0..6 {
        automaton.step();
    }

    println!("Day 17, Part 1: {}", automaton.count_active());
}
