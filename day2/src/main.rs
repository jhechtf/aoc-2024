use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Values");
    let lines = contents.lines();

    let mapped_lines = lines
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|level| {
            (0..level.len()).any(|i| {
                let mut level = level.clone();
                level.remove(i);
                is_safe_level(level)
            })
        });

    println!("{}", mapped_lines.count());
}

enum Trend {
    Uknown,
    Increasing,
    Decreasing,
}

fn is_safe_level(levels: Vec<i32>) -> bool {
    let mut trend = Trend::Uknown;
    for w in levels.windows(2) {
        let x = w[0];
        let y = w[1];

        if x == y {
            return false;
        }

        match trend {
            Trend::Increasing if x > y => return false,
            Trend::Decreasing if x < y => return false,
            Trend::Uknown if x > y => trend = Trend::Decreasing,
            Trend::Uknown if x < y => trend = Trend::Increasing,
            _ => {}
        }

        let diff = x.abs_diff(y);
        if !(0..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn map_levels(levels: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];

    for i in 1..levels.len() {
        ret.push(levels[i] - levels[i - 1]);
    }

    ret
}
