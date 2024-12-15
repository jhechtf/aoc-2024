mod part1;
mod shared;

const TEST: &str = include_str!("../test.txt");

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_fake() {
        assert_eq!(part1::part1(TEST), 0);
    }
}
