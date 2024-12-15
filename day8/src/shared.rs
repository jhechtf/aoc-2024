use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct Point<T> {
    value: T,
    x: i32,
    y: i32,
}

impl<T> Point<T> {
    fn new(value: T, x: i32, y: i32) -> Self {
        Self { value, x, y }
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

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.values.get(&(x, y));
                if let Some(value) = value {
                    write!(f, "{value}");
                } else {
                    write!(f, ".");
                }
            }
            write!(f, "\n");
        }
    }
}
