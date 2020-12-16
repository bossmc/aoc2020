#[derive(Debug, Clone, Copy)]
struct Mask {
    one: u64,
    zero: u64,
    x: u64,
}

impl Mask {
    fn new() -> Self {
        Mask {
            one: 0,
            zero: 0,
            x: 0,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Mask(Mask),
    Write { addr: u64, value: u64 },
}

fn load_data() -> Vec<Operation> {
    crate::util::data_lines(14)
        .map(|line| {
            match &line[0..3] {
                "mas" => {
                    let mask = line.as_bytes()[7..].iter().enumerate().fold(
                        Mask::new(),
                        |mut acc, (idx, c)| {
                            let shift = 35 - idx;
                            match c {
                                b'1' => acc.one |= 1u64 << shift,
                                b'0' => acc.zero |= 1u64 << shift,
                                b'X' => acc.x |= 1u64 << shift,
                                _ => unreachable!(),
                            }
                            acc
                        },
                    );
                    Operation::Mask(mask)
                }
                "mem" => {
                    let mut addr = 0;
                    let mut idx = 4; // Skip over "mem["
                    loop {
                        let c = line.as_bytes()[idx];
                        if c == b']' {
                            break;
                        } else {
                            addr *= 10;
                            addr += (c - b'0') as u64;
                            idx += 1;
                        }
                    }

                    let value = std::str::FromStr::from_str(&line[idx + 4..]).unwrap(); // Skip over "] = "
                    Operation::Write { addr, value }
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1() {
    let data = load_data();
    let mut mem = std::collections::BTreeMap::<u64, u64>::new();
    let mut mask = Mask::new();
    for op in data.iter() {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Write { addr, value } => {
                mem.insert(*addr, (*value | mask.one) & !mask.zero);
            }
        }
    }
    let memory_sum: u64 = mem.iter().map(|(_k, v)| v).sum();
    println!("Day 14, Part 1: {}", memory_sum);
}

pub fn part2() {
    let data = load_data();
    let mut mem = std::collections::BTreeMap::<u64, u64>::new();
    let mut mask = Mask::new();
    for op in data.iter() {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Write { addr, value } => {
               let mut written_addresses = std::collections::BTreeSet::new();
               written_addresses.insert(0);
               for idx in 0..36 {
                   let bit = 1 << idx;
                   if mask.one & bit == bit {
                       written_addresses = written_addresses.into_iter().map(|w_addr| {
                           w_addr | bit
                       }).collect();
                   } else if mask.zero & bit == bit {
                       written_addresses = written_addresses.into_iter().map(|w_addr| {
                           w_addr | (addr & bit)
                       }).collect();
                   } else if mask.x & bit == bit {
                       written_addresses = written_addresses.into_iter().flat_map(move |w_addr| {
                           vec![w_addr, w_addr | bit].into_iter()
                       }).collect();
                   } else {
                       unreachable!("Bit not in one/zero/x");
                   }
               }
               for addr in written_addresses {
                   mem.insert(addr, *value);
               }
            }
        }
    }
    let memory_sum: u64 = mem.iter().map(|(_k, v)| v).sum();
    println!("Day 14, Part 2: {}", memory_sum);
}
