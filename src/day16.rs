use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ticket {
    fields: [usize; 20],
}

impl Ticket {
    fn find_invalid_field(&self, rules: &HashSet<Rule>) -> Option<usize> {
        for field in &self.fields {
            if !rules.iter().any(|rule| rule.accepts(field)) {
                return Some(*field);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    name: &'static str,
    low_range: RangeInclusive<usize>,
    high_range: RangeInclusive<usize>,
}

impl Rule {
    fn accepts(&self, field: &usize) -> bool {
        self.low_range.contains(field) || self.high_range.contains(field)
    }
}

fn rules() -> HashSet<Rule> {
    let mut rules = HashSet::new();
    rules.insert(Rule {
        name: "departure location",
        low_range: 26..=724,
        high_range: 743..=964,
    });
    rules.insert(Rule {
        name: "departure station",
        low_range: 33..=845,
        high_range: 864..=954,
    });
    rules.insert(Rule {
        name: "departure platform",
        low_range: 26..=472,
        high_range: 482..=967,
    });
    rules.insert(Rule {
        name: "departure track",
        low_range: 27..=140,
        high_range: 158..=956,
    });
    rules.insert(Rule {
        name: "departure date",
        low_range: 25..=884,
        high_range: 894..=952,
    });
    rules.insert(Rule {
        name: "departure time",
        low_range: 37..=924,
        high_range: 941..=949,
    });
    rules.insert(Rule {
        name: "arrival location",
        low_range: 48..=311,
        high_range: 335..=972,
    });
    rules.insert(Rule {
        name: "arrival station",
        low_range: 39..=703,
        high_range: 724..=950,
    });
    rules.insert(Rule {
        name: "arrival platform",
        low_range: 40..=108,
        high_range: 114..=950,
    });
    rules.insert(Rule {
        name: "arrival track",
        low_range: 30..=101,
        high_range: 108..=967,
    });
    rules.insert(Rule {
        name: "class",
        low_range: 33..=386,
        high_range: 399..=949,
    });
    rules.insert(Rule {
        name: "duration",
        low_range: 44..=444,
        high_range: 452..=956,
    });
    rules.insert(Rule {
        name: "price",
        low_range: 27..=220,
        high_range: 234..=974,
    });
    rules.insert(Rule {
        name: "route",
        low_range: 42..=774,
        high_range: 790..=959,
    });
    rules.insert(Rule {
        name: "row",
        low_range: 48..=900,
        high_range: 918..=956,
    });
    rules.insert(Rule {
        name: "seat",
        low_range: 42..=165,
        high_range: 178..=949,
    });
    rules.insert(Rule {
        name: "train",
        low_range: 45..=831,
        high_range: 842..=965,
    });
    rules.insert(Rule {
        name: "type",
        low_range: 49..=522,
        high_range: 548..=974,
    });
    rules.insert(Rule {
        name: "wagon",
        low_range: 32..=565,
        high_range: 588..=964,
    });
    rules.insert(Rule {
        name: "zone",
        low_range: 27..=608,
        high_range: 617..=953,
    });
    rules
}

fn my_ticket() -> Ticket {
    Ticket {
        fields: [
            101, 71, 193, 97, 131, 179, 73, 53, 79, 67, 181, 89, 191, 137, 163, 83, 139, 127, 59,
            61,
        ],
    }
}

fn other_tickets() -> HashSet<Ticket> {
    use std::convert::TryInto as _;
    crate::util::data_lines(16)
        .map(|line| Ticket {
            fields: line
                .split(',')
                .map(|entry| std::str::FromStr::from_str(entry).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
        .collect()
}

pub fn part1() {
    let rules = rules();
    let other_tickets = other_tickets();
    let invalid_fields = other_tickets
        .iter()
        .flat_map(|ticket| ticket.find_invalid_field(&rules));
    println!("Day 16, Part 1: {}", invalid_fields.sum::<usize>());
}

pub fn part2() {
    let my_ticket = my_ticket();
    let rules = rules();
    let other_tickets = other_tickets()
        .into_iter()
        .filter(|ticket| ticket.find_invalid_field(&rules).is_none())
        .collect::<HashSet<_>>();

    // Work out which fields could be controlled by a rule (i.e. other_tickets contains no counter-examples)
    let mut valid_fields_by_rule = std::collections::HashMap::<&'static str, Vec<usize>>::new();
    for field in 0..20 {
        for rule in &rules {
            if other_tickets
                .iter()
                .all(|ticket| rule.accepts(&ticket.fields[field]))
            {
                valid_fields_by_rule
                    .entry(rule.name)
                    .or_default()
                    .push(field);
            }
        }
    }

    // Naively assign each rule to a field if that's the only field that (a) matches the rule and
    // (b) has not already been assigned to another rule.
    //
    // This assumes there's a unique solution.
    let rule_names = rules.iter().map(|rule| rule.name).collect::<Vec<_>>();
    let mut assigned_fields = std::collections::HashSet::<usize>::new();
    let mut rule_to_field = std::collections::HashMap::<&'static str, usize>::new();
    while assigned_fields.len() != 20 {
        for name in &rule_names {
            let mut possible_rules = valid_fields_by_rule[name]
                .iter()
                .filter(|field| !assigned_fields.contains(field));
            if let (Some(field), None) = (possible_rules.next(), possible_rules.next()) {
                assigned_fields.insert(*field);
                rule_to_field.insert(name, *field);
            }
        }
    }

    // Calculate the final answer
    let mut product = 1;
    for name in rule_names {
        if name.starts_with("departure") {
            let field = rule_to_field[name];
            product *= my_ticket.fields[field];
        }
    }

    println!("Day 16, Part 2: {}", product);
}
