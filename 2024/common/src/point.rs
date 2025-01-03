use std::ops::{Add, Mul, Sub};

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
pub struct Point(pub isize, pub isize);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Point> for isize {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
    }
}

impl Mul<&Point> for isize {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
    }
}
