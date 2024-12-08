use std::collections::HashMap;
use std::ops::Mul;

fn main() {
    println!("Hello, world!");
}

// A point in the grid, contains x + y and the value
#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: isize,
    pub y: isize,
    pub value: char,
}

impl Point {
    fn get(&self, direction: (isize, isize)) -> Point {
        let mut new_point = Self {
            x: self.x,
            y: self.y,
            value: self.value,
        };

        new_point.x += direction.0;
        new_point.y += direction.1;

        new_point
    }
}

fn to_point_grid(input: &str) -> Grid {
    Grid::new(input)
}

struct Grid {
    pub grid: HashMap<(isize, isize), Point>,
    pub width: isize,
    pub height: isize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut g = HashMap::new();
        let chars: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        for (y, row) in chars.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let point = (x as isize, y as isize);
                g.insert(
                    point,
                    Point {
                        x: x as isize,
                        y: y as isize,
                        value: *c,
                    },
                );
            }
        }

        Grid {
            grid: g,
            height: chars.len() as isize,
            width: chars[0].len() as isize,
        }
    }

    fn get(&self, point: (isize, isize)) -> Option<&Point> {
        return self.grid.get(&point);
    }

    fn move_from(&self, point: Point, direction: (isize, isize)) -> Option<&Point> {
        let new_coords = (point.x + direction.0, point.y + direction.1);
        let new_point = self.get(new_coords);
        new_point
    }
}

fn part1(input: &str) -> i32 {
    // turn the str into a vec of vecs of chars
    let grid = to_point_grid(input);
    let mut count = 0;

    let directions: [(isize, isize); 8] = [
        // To right
        (0, 1),
        // To left
        (0, -1),
        // To up
        (-1, 0),
        // To down
        (1, 0),
        // To right down
        (1, 1),
        // To left down
        (1, -1),
        // to right up
        (-1, 1),
        // to left up
        (-1, -1),
    ];

    for ((x, y), point) in grid.grid.iter() {
        let f = 0 as isize;
        let g = 5 as isize;
        if point.value == 'X' {
            for dir in directions.iter() {
                // This is truly the jankest shit i've ever written
                if let Some(p2) = grid.move_from(*point, *dir) {
                    if p2.value == 'M' {
                        if let Some(p3) = grid.move_from(*p2, *dir) {
                            if p3.value == 'A' {
                                if let Some(p4) = grid.move_from(*p3, *dir) {
                                    if p4.value == 'S' {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn mult_by_magnitude(a: isize, b: (isize, isize)) -> (isize, isize) {
    (a * b.0, a * b.1)
}

fn part2(input: &str) -> i32 {
    let grid = to_point_grid(input);
    let mut count = 0;
    for ((x, y), point) in grid.grid.iter() {
        if point.value == 'A' {
            println!("{:?}", point);
            if let Some(c1) = grid.move_from(*point, ( -1, -1)) {
                if let Some(c2) = grid.move_from(*point, (-1, 1)) {
                    // We've got the two corners, check their opposites;
                    let c3 = grid.move_from(*point, (1, 1));
                    let c4 = grid.move_from(*point, (1, -1));

                    if c3.is_some() && c4.is_some() {
                        let c3_value = c3.unwrap();
                        let c4_value = c4.unwrap();
                        let pairs: [(char, char); 2] =
                            [(c1.value, c3_value.value), (c2.value, c4_value.value)];

                        println!("{:?}: {:?}", point, pairs);

                        match pairs {
                            [('S', 'M'), ('S', 'M')]
                            | [('S', 'M'), ('M', 'S')]
                            | [('M', 'S'), ('S', 'M')]
                            | [('M', 'S'), ('M', 'S')] => {
                                println!("FOUND A MATCH");
                                count += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_fake() {
        let data = include_str!("../test.txt");
        assert_eq!(part1(data), 18);
    }

    #[test]
    fn test_part1_real() {
        let data = include_str!("../input.txt");
        assert_eq!(part1(data), 2547);
    }

    #[test]
    fn test_part2_fake() {
        let data = include_str!("../test.txt");
        assert_eq!(part2(data), 9);
    }
    #[test]
    fn test_part2_real() {
        let data = include_str!("../input.txt");
        assert_eq!(part2(data), 1856);
    }
}
