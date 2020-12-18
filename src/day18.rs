fn load_data() -> Vec<Vec<u8>> {
    crate::util::data_lines(18)
        .map(|line| line.bytes().filter(|b| !b.is_ascii_whitespace()).collect())
        .collect()
}

/// A stack-based APU to be driven by the shunting yard algorithm
struct Output(Vec<usize>);

impl Output {
    fn push_value(&mut self, v: usize) {
        self.0.push(v);
    }

    fn apply_operation(&mut self, op: u8) {
        let arg1 = self.0.pop().expect("Insufficient args for operation");
        let arg2 = self.0.pop().expect("Insufficient args for operation");
        self.push_value(match op {
            b'+' => arg1 + arg2,
            b'*' => arg1 * arg2,
            _ => unreachable!(),
        });
    }

    fn finish(mut self) -> usize {
        let result = self.0.pop().expect("No expression");
        assert!(self.0.is_empty());
        result
    }
}

fn shunting_yard(input: impl Iterator<Item = u8>, precedence: fn(u8) -> u8) -> usize {
    let mut output = Output(Vec::new());
    let mut siding = Vec::new();
    for token in input {
        match token {
            b'0'..=b'9' => output.push_value((token - b'0') as usize),
            b'+' | b'*' => {
                // About to process an operation, first, apply all previously seen,
                // higher-priority operations (in the current bracket term).
                while let Some(op) = siding.pop() {
                    if op == b'(' || precedence(op) < precedence(token) {
                        siding.push(op);
                        break;
                    } else {
                        output.apply_operation(op);
                    }
                }
                // Defer the actual processing of this operation until a lower-priority operation
                // is met.
                siding.push(token);
            }
            b'(' => siding.push(token),
            b')' => {
                // Apply all outstanding operation from this bracketed term
                loop {
                    match siding.pop() {
                        Some(b'(') => {
                            // Found the '(' to this ')' - discard it
                            break;
                        }
                        Some(op) => output.apply_operation(op),
                        None => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    // Apply all outstanding operations to complete the calculation
    while let Some(op) = siding.pop() {
        output.apply_operation(op);
    }
    output.finish()
}

pub fn part1() {
    fn part_1_precedence(_op: u8) -> u8 {
        // All operations are left-associative and of equal priority
        0
    }

    println!(
        "Day 18, Part 1: {}",
        load_data()
            .into_iter()
            .map(|problem| shunting_yard(problem.into_iter(), part_1_precedence))
            .sum::<usize>()
    );
}

pub fn part2() {
    fn part_2_precedence(op: u8) -> u8 {
        match op {
            // All operations are left-associative but `+` binds harder than `*`
            b'+' => 2,
            b'*' => 1,
            _ => unreachable!(),
        }
    }

    println!(
        "Day 18, Part 2: {}",
        load_data()
            .into_iter()
            .map(|problem| shunting_yard(problem.into_iter(), part_2_precedence))
            .sum::<usize>()
    );
}
