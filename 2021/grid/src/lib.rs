use anyhow::Result;
use std::io;
use std::io::BufRead;

/// A grid used to store values and access elements via neighbors
pub struct Grid {
    pub storage: Vec<usize>,
    pub height: usize,
    pub length: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub value: usize,
}

impl Grid {
    pub fn from_stdin(length: usize, height: usize) -> Result<Grid> {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let lines = handle.lines();

        let mut storage = Vec::new();

        for line in lines {
            let line = line?;
            storage.append(
                &mut line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>(),
            )
        }
        Ok(Grid {
            length,
            height,
            storage,
        })
    }

    pub fn from_vec_str(length: usize, height: usize, lines: Vec<String>) -> Result<Grid> {
        let mut storage: Vec<usize> = Vec::new();
        for line in lines {
            storage.append(
                &mut line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>(),
            )
        }
        Ok(Grid {
            length,
            height,
            storage,
        })
    }

    pub fn access_mut(&mut self, x: isize, y: isize) -> Option<&mut usize> {
        match self.storage_index(x, y) {
            Some(value) => Some(&mut self.storage[value]),
            None => None,
        }
    }

    /// Given a position, access the value directly
    /// DEPRECATED (lol its funny to write this in a personal project)
    pub fn access(&self, x: isize, y: isize) -> Option<usize> {
        self.storage_index(x, y).map(|value| self.storage[value])
    }
    /// Access, but using the point as a public structure
    pub fn get_point(&self, x: isize, y: isize) -> Option<Point> {
        self.access(x, y).map(|value| Point { x, y, value })
    }

    fn storage_index(&self, x: isize, y: isize) -> Option<usize> {
        if x >= self.length as isize || y >= self.height as isize || x < 0 || y < 0 {
            None
        } else {
            Some((y * self.length as isize + x) as usize)
        }
    }
}
