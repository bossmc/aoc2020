use std::collections::{HashMap, HashSet};

struct Trees {
    forward: HashMap<String, HashSet<(usize, String)>>,
    backward: HashMap<String, HashSet<String>>,
}

fn load_data() -> Trees {
    let mut backward = HashMap::new();
    let forward = crate::util::data_lines(7).map(|line| {
        let (outer, rest) = <str as crate::util::StrExt>::split_once(&line, " bags contain ");
        if rest == "no other bags." {
            (outer.to_string(), HashSet::new())
        } else {
            let rest = rest.trim_end_matches(".");
            let inner = rest.split(", ").map(|term| {
                let term = term.trim_end_matches("s");
                let term = term.trim_end_matches(" bag");
                let (count, colour) = <str as crate::util::StrExt>::split_once(term, " ");
                backward.entry(colour.to_string()).or_insert_with(|| HashSet::new()).insert(outer.to_string());
                (<usize as std::str::FromStr>::from_str(count).unwrap(), colour.to_string())
            }).collect();
            (outer.to_string(), inner)
        }
    }).collect();
    Trees {
        forward,
        backward,
    }
}

pub fn part1() {
    let data = load_data();
    let mut processed = HashSet::new();
    processed.insert("shiny gold".to_string());
    let mut to_process = vec!["shiny gold".to_string()];
    while let Some(current) = to_process.pop() {
        if let Some(next_colours) = data.backward.get(&current) {
            for colour in next_colours {
                if !processed.contains(colour) {
                    to_process.push(colour.clone());
                    processed.insert(colour.clone());
                }
            }
        }
    }
    println!("Day 7, Part 1: {}", processed.len() - 1); // Subtract 1 because we primed the set with shiny gold which isn't a valid outer colour
}

pub fn part2() {
    let data = load_data();

    fn count_inner_bags(trees: &Trees, name: &str) -> usize {
        trees.forward[name].iter().fold(0, |acc, (count, name)| {
            acc + count * count_inner_bags(trees, name) + count
        })
    }

    println!("Day 7, Part 2: {}", count_inner_bags(&data, "shiny gold"));
}

