use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    time::Instant,
};
const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");
fn main() {
    let mut start = Instant::now();
    part1(INPUT);
    println!("{:?}", start.elapsed());
    start = Instant::now();
    println!("{:?}", start.elapsed());
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, Clone)]
struct Grid {
    values: HashMap<(i32, i32), char>,
    player: (i32, i32),
    direction: Direction,
    width: isize,
    height: isize,
}

impl Grid {
    fn get_next_coord(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => {
                // println!("UP {pos:?}");
                let (y, x) = self.player;
                (y - 1, x)
            }
            Direction::Right => {
                // println!("RIGHT {pos:?}");
                let (y, x) = self.player;
                (y, x + 1)
            }
            Direction::Down => {
                // println!("DOWN {pos:?}");
                let (y, x) = self.player;
                (y + 1, x)
            }
            Direction::Left => {
                // println!("LEFT {pos:?}");
                let (y, x) = self.player;
                (y, x - 1)
            }
        }
    }
    // Returns the new direction the plguard is facing, modifying the current direction
    fn rotate(&mut self) -> Direction {
        let ret = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.direction = ret;
        ret
    }
    pub fn walk(&mut self) {
        // Logically: we iterate the y value down. if it goes negative we stop.
        // if facing right we iterate the x value up. if it goes over the size of the grid we stop.
        // if facing down we increase the y value up. If it goes over the size of the grid we stop.
        // if facing left we iterate the x value down. if it goes negative we stop.

        loop {
            let new_pos_coord = self.get_next_coord();
            let new_pos = self.values.get(&new_pos_coord);
            // println!("{new_pos:?}");
            // If the new pos doesn't exist, we're out of bounds
            if new_pos.is_none() {
                break;
            }

            let new_pos = new_pos.unwrap();

            if *new_pos == '#' {
                self.rotate();
            } else {
                self.values.entry(new_pos_coord).and_modify(|v| *v = 'X');
                self.player = new_pos_coord;
            }
        }
    }

    pub fn count_tiles(&self) -> i32 {
        self.values.values().filter(|v| **v == 'X').count() as i32
    }

    // additions for part 2
    // Okay, so we're going to go with a very stupid implementation because I'm lazy.
    pub fn count_loop_points(&mut self) -> i32 {
        let mut count = 0;
        // It does not matter if we go in order of the points
        for dir in self.values.keys() {
            let mut clone = self.clone();
            clone.values.entry(*dir).and_modify(|v| *v = '#');

            if clone.is_infinite_loop() {
                count += 1;
            }
        }
        count
    }

    fn is_infinite_loop(&mut self) -> bool {
        // Keep track of the directio
        let mut walked: HashMap<Direction, HashSet<(i32, i32)>> = HashMap::new();
        loop {
            let is_walked_available = walked
                .entry(self.direction)
                .or_insert(HashSet::new())
                .contains(&self.player);
            if is_walked_available {
                return true;
            }
            let new_pos_coord = self.get_next_coord();
            let new_pos = self.values.get(&new_pos_coord);

            // If the new pos doesn't exist, we're out of bounds
            if new_pos.is_none() {
                break;
            }

            let new_pos = new_pos.unwrap();

            if *new_pos == '#' {
                self.rotate();
            } else {
                self.values.entry(new_pos_coord).and_modify(|v| *v = 'X');
                walked.entry(self.direction).and_modify(|v| {
                    v.insert(self.player);
                });
                self.player = new_pos_coord;
            }
        }
        false
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                s.push_str(&format!(
                    "{}",
                    self.values.get(&(i as i32, j as i32)).unwrap()
                ));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut grid: HashMap<(i32, i32), char> = HashMap::new();

        let mut start = (0, 0);
        let mut dir = Direction::Up;
        let mut height = 0;
        let mut width = 0;
        let points: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
                // line.split(" ")
                //     .map(|v| v.chars())
                //     .collect::<Vec<char>>()
            })
            .collect();
        // Iterate over the rows and columns
        for (i, row) in points.iter().enumerate() {
            height = i as isize;
            for (j, cell) in row.iter().enumerate() {
                width = j as isize;
                // insert the item into the grid
                grid.insert((i as i32, j as i32), *cell);
                // Match the contents of the cell
                match *cell {
                    // Not necessary technically but like w/e
                    '^' | '>' | '<' | 'v' => {
                        start = (i as i32, j as i32);
                        // Set the direction to be the correct setup.
                        dir = match *cell {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            _ => Direction::Up,
                        }
                    }
                    _ => {}
                }
            }
        }

        Self {
            values: grid,
            player: start,
            direction: dir,
            height,
            width,
        }
    }
}

/**
 * Parse the grid into a 2D vector of i32s.
 * Returns the grid and the starting position of the guard
 */
fn parse_grid(input: &str) -> Grid {
    Grid::from(input)
}

fn part1(input: &str) -> i32 {
    // Parse the grid
    let mut grid = parse_grid(input);
    grid.walk();
    grid.count_tiles()
}

fn part2(input: &str) -> i32 {
    let mut grid = parse_grid(input);
    let f = grid.count_loop_points();
    f
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_fake() {
        assert_eq!(part1(TEST), 41);
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(part1(INPUT), 5269);
    }

    #[test]
    fn test_part2_fake() {
        assert_eq!(part2(TEST), 6);
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(part2(INPUT), 0);
    }
}
