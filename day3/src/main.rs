use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut go = true;
    let mut working = String::new();

    let operator_regex = Regex::new(r"(don't\(\)|do\(\))$").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for i in input.chars() {
        working.push(i);
        if i == ')' {
            if working.ends_with("do()") {
                go = true;
            } else if working.ends_with("don't()") {
                go = false;
            } else if working.contains("mul(") {
                if go {
                    if let Some(caps) = re.captures(&working) {
                        let x = caps.get(1).map_or("", |m| m.as_str());
                        let y = caps.get(2).map_or("", |m| m.as_str());
                        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
                    }
                }
            }

            working.clear();
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1_fake() {
        let data = read_to_string("./test.txt").expect("Value");
        let ret = part1(&data);
        assert_eq!(ret, 161);
    }

    #[test]
    fn test_part1_real() {
        let data = read_to_string("./input.txt").expect("Values");
        let ret = part1(&data);
        assert_eq!(ret, 169021493);
    }

    #[test]
    fn test_part2_fake() {
        let data = read_to_string("./test.txt").expect("Value");
        let ret = part2(&data);
        assert_eq!(ret, 48);
    }

    #[test]
    fn test_part2_real() {
        let data = read_to_string("./input.txt").expect("Values");
        let ret = part2(&data);
        assert_eq!(ret, 111762583);
    }
}
