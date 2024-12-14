mod part1;
mod part2;
use part1::part1;
use part2::part2;

const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");

fn main() {
    println!("Hello, world!");
    let start = std::time::Instant::now();
    part1(INPUT);
    println!("part1 took {}ms", start.elapsed().as_millis());
    let start = std::time::Instant::now();
    part2(INPUT);
    println!("part2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_fake() {
        assert_eq!(part1(TEST), 3749);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 4555081946288);
    }

    #[test]
    fn part2_fake() {
        assert_eq!(part2(TEST), 11387);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 0);
    }
}
