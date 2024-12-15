use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    value: T,
    x: i32,
    y: i32,
}

impl<T> Point<T> {
    fn new(value: T, x: i32, y: i32) -> Self {
        Self { value, x, y }
    }

    pub fn is_in_bounds(&self, grid: &Grid<T>) -> bool {
        between(self.x, [0, grid.width]) && between(self.y, [0, grid.height])
    }
}

impl<T: Copy> Point<T> {
    pub fn add(&self, other: &Point<T>) -> Point<T> {
        let point = Point::new(self.value, self.x + other.x, self.y + other.y);
        point
    }
}

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self::new(T::default(), 0, 0)
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    values: HashMap<(i32, i32), Point<T>>,
    width: i32,
    height: i32,
}

impl<T> Grid<T> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            values: HashMap::new(),
            width,
            height,
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn set(&mut self, coord: (i32, i32), value: T) {
        self.values
            .entry(coord)
            .and_modify(|v| v.value = value)
            .or_insert(Point::new(value, coord.0, coord.1));
    }

    pub fn get(&self, coord: (i32, i32)) -> Option<&Point<T>> {
        self.values.get(&coord)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!(
                    "{}",
                    match self.values.get(&(y, x)) {
                        Some(point) => point.value.to_string(),
                        None => ".".to_string(),
                    }
                ));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

pub fn between(value: i32, min_max: [i32; 2]) -> bool {
    min_max[0] <= value && value <= min_max[1]
}
