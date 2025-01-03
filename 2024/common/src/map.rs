use anyhow::Result;
use std::io::{BufRead, Lines};

use crate::point::Point;

pub struct Map2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Map2D<char> {
    pub fn load<T>(lines: Lines<T>) -> Result<Self>
    where
        T: BufRead,
    {
        let mut width: Option<usize> = None;
        let mut height = 0usize;
        let mut data = vec![];
        for line_raw in lines {
            let line = line_raw?;
            let line = line.trim();
            if width.is_none() {
                width = Some(line.len());
            }
            for character in line.chars() {
                data.push(character);
            }
            height += 1;
        }

        Ok(Self {
            data,
            width: width.unwrap(),
            height,
        })
    }
}

impl<From> Map2D<From> {
    pub fn map<Into>(self, predicate: impl FnMut(&From) -> Result<Into>) -> Result<Map2D<Into>> {
        let res: Result<Vec<_>, _> = self.data.iter().map(predicate).collect();
        Ok(Map2D::<Into> {
            data: res?,
            width: self.width,
            height: self.height,
        })
    }
}

impl<T> Map2D<T> {
    pub fn for_each(&self, callback: &mut impl FnMut(Point, &T) -> Result<()>) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point(x as isize, y as isize);
                let val = self.get(&point);
                callback(point, val)?;
            }
        }
        Ok(())
    }

    pub fn is_on_map(&self, point: &Point) -> bool {
        point.0 >= 0
            && point.0 < self.width as isize
            && point.1 >= 0
            && point.1 < self.height as isize
    }

    pub fn get(&self, point: &Point) -> &T {
        &self.data[(point.0 + point.1 * self.width as isize) as usize]
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
