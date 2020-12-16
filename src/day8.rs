use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
    Term,
}

fn load_data() -> Rc<[Op]> {
    let mut program = crate::util::data_lines(8)
        .map(|line| {
            use std::str::FromStr as _;
            let (op, sign, num) = (&line[..3], &line.as_bytes()[3], &line[4..]);
            let mut arg = isize::from_str(num).unwrap();
            if *sign == b'-' {
                arg *= -1;
            }
            match op {
                "nop" => Op::Nop(arg),
                "acc" => Op::Acc(arg),
                "jmp" => Op::Jmp(arg),
                _ => unreachable!("{:?}", line),
            }
        })
        .collect::<Vec<_>>();
    program.push(Op::Term);
    program.into()
}

struct State {
    pc: isize,
    acc: isize,
    visited: std::collections::HashSet<isize>,
    program: Rc<[Op]>,
    tweaked_address: Option<isize>,
}

enum PostStep {
    State(State),
    Termination(isize),
    LiveLock(isize),
}

enum PostRun {
    Termination(isize),
    LiveLock(isize),
}

impl State {
    fn new(program: Rc<[Op]>) -> Self {
        State {
            pc: 0,
            acc: 0,
            visited: Default::default(),
            program,
            tweaked_address: None,
        }
    }

    fn tweak(&mut self, addr: isize) {
        self.tweaked_address = Some(addr);
    }

    fn step(mut self) -> PostStep {
        if self.visited.contains(&self.pc) {
            return PostStep::LiveLock(self.acc);
        }

        self.visited.insert(self.pc);
        assert!(self.pc >= 0);

        let mut op = self.program[self.pc as usize];
        if self.tweaked_address == Some(self.pc) {
            op = match op {
                Op::Jmp(arg) => Op::Nop(arg),
                Op::Nop(arg) => Op::Jmp(arg),
                op => op,
            }
        }
        match op {
            Op::Nop(_) => PostStep::State(State {
                pc: self.pc + 1,
                ..self
            }),
            Op::Acc(arg) => PostStep::State(State {
                acc: self.acc + arg,
                pc: self.pc + 1,
                ..self
            }),
            Op::Jmp(arg) => PostStep::State(State {
                pc: self.pc + arg,
                ..self
            }),
            Op::Term => PostStep::Termination(self.acc),
        }
    }

    fn run(mut self) -> PostRun {
        loop {
            self = match self.step() {
                PostStep::State(state) => state,
                PostStep::LiveLock(acc) => return PostRun::LiveLock(acc),
                PostStep::Termination(acc) => return PostRun::Termination(acc),
            };
        }
    }
}

pub fn part1() {
    let program = load_data();
    let state = State::new(program);
    match state.run() {
        PostRun::LiveLock(acc) => println!("Day 8, Part 1: {}", acc),
        PostRun::Termination(_) => unreachable!(),
    };
}

pub fn part2() {
    let program = load_data();
    let program_length = program.len();
    for idx in 0..program_length {
        let mut state = State::new(Rc::clone(&program));
        state.tweak(idx as isize);
        if let PostRun::Termination(acc) = state.run() {
            println!("Day 8, Part 2: {}", acc);
            return;
        }
    }
    unreachable!("Didn't find an answer");
}
