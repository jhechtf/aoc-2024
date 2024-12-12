use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter::Map,
    path::Iter,
    time::Instant,
};

// Test input and real input data to make the tests
const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut start = Instant::now();
    part1(INPUT);
    println!("{:?}", start.elapsed());

    start = Instant::now();
    part2(TEST);
    println!("{:?}", start.elapsed());
}

fn part1(input: &str) -> i32 {
    let split: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(split[0]);
    let values = parse_edits(split[1], &rules);
    values
}

fn parse_rules(input: &str) -> HashMap<i32, Vec<Rule>> {
    let mut map: HashMap<i32, Vec<Rule>> = HashMap::new();

    let parts = input.split_whitespace().map(|line| {
        let nums: Vec<i32> = line.split("|").map(|n| n.parse().unwrap()).collect();
        (nums[0], nums[1])
    });

    for (before, after) in parts {
        map.entry(before)
            .and_modify(|v| {
                v.push(Rule::Before(after));
            })
            .or_insert_with(|| vec![Rule::Before(after)]);

        map.entry(after)
            .and_modify(|v| {
                v.push(Rule::After(before));
            })
            .or_insert_with(|| vec![Rule::After(before)]);
    }
    map
}

fn parse_edits(input: &str, rules: &HashMap<i32, Vec<Rule>>) -> i32 {
    input
        .split_whitespace()
        .filter(|v| validate_prints(v, rules))
        .map(|r| {
            let values: Vec<i32> = r.split(",").map(|v| v.parse::<i32>().unwrap()).collect();
            values[values.len() / 2]
        })
        .fold(0, |acc, v| acc + v)
    // .collect()
}

fn validate_prints(input: &str, rules: &HashMap<i32, Vec<Rule>>) -> bool {
    let vals: Vec<i32> = input
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", vals);

    for (i, val) in vals.iter().enumerate() {
        // We either have rules or we continue the loop.
        let Some(rules_for_num) = rules.get(&val) else {
            continue;
        };

        for vv in &vals[i + 1..] {
            for rule in rules_for_num {
                match rule {
                    Rule::After(n) => {
                        if n == vv {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    true
}

fn invalidate_prints(input: &str, rules: &HashMap<i32, Vec<Rule>>) -> bool {
    println!("Invalidate prints:\n\t {:?}", input);
    !validate_prints(input, rules)
}

fn fix_errors(input: &str, rules: &HashMap<i32, Vec<Rule>>) -> Vec<i32> {
    let mut ret = Vec::from_iter(input.split(",").map(|v| v.parse::<i32>().unwrap_or(0)));
    ret.sort_by(|a, b| {
        let a_rules_raw = rules.get(a);
        if let Some(a_rules) = a_rules_raw {
            for rule in a_rules {
                match rule {
                    Rule::After(x) => {
                        if x == b {
                            return Ordering::Greater;
                        }
                    }
                    Rule::Before(x) => {
                        if x == b {
                            return Ordering::Less;
                        }
                    }
                }
            }
        }
        Ordering::Equal
    });
    ret
}

fn parse_edits_for_fixing(input: &str, rules: &HashMap<i32, Vec<Rule>>) -> i32 {
    input
        .split_whitespace()
        .filter(|v| invalidate_prints(v, rules))
        .map(|line| fix_errors(line, rules))
        .fold(0, |acc, line| acc + line[line.len() / 2])
}

fn part2(input: &str) -> i32 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_rules(parts[0]);
    println!("Parts {:?}", parts[1]);
    let bob = parse_edits_for_fixing(parts[1], &rules);
    println!("BOB {}", bob);
    bob
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Rule {
    Before(i32),
    After(i32),
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_part1_fake() {
    //     assert_eq!(part1(TEST), 143);
    // }

    // #[test]
    // fn test_part1_real() {
    //     assert_eq!(part1(INPUT), 4774);
    // }

    #[test]
    fn test_part2_fake() {
        assert_eq!(part2(TEST), 123);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(INPUT), 0);
    }
}
