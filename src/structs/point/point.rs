use super::PointWithItem;

/// A simple single coordinate
#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(value: (usize, usize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
    /// a way to add an item to a coordinate.
    /// ```
    /// # use arena::structs::point::Point;
    /// # let point = Point {x : 0, y : 0};
    /// assert_eq!(point.add_item(10).item,10);
    /// ```
    pub fn add_item<T>(&self, item: T) -> PointWithItem<T> {
        PointWithItem {
            x: self.x,
            y: self.y,
            item,
        }
    }
    /// A way to get all points between two points. Usefull to draw lines over the grid
    /// Internally, it uses make_horizontal_line, make_vertical_line and make_diagonal_line depending on what kind of line is needed.
    /// # Examples
    /// Horizontal line:
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:3,y:0};
    /// let line = point.make_line(point2);
    /// for coordinate in 0 ..= 3 {
    ///     assert!(line[coordinate].x == coordinate);
    /// }
    /// ```
    /// Vertical line:
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:0,y:3};
    /// let line = point.make_line(point2);
    /// for coordinate in 0 ..= 3 {
    ///     assert!(line[coordinate].y == coordinate);
    /// }
    /// ```
    pub fn make_line(self, other: Point) -> Vec<Point> {
        if other.x != self.x && self.y == other.y {
            if self.x > other.x {
                let diff = 1 + self.x - other.x;
                other.make_horizontal_line(diff)
            } else {
                let diff = 1 + other.x - self.x;
                self.make_horizontal_line(diff)
            }
        } else if self.y > other.y {
            let diff = 1 + self.y - other.y;
            other.make_vertical_line(diff)
        } else {
            let diff = 1 + other.y - self.y;
            self.make_vertical_line(diff)
        }
    }
    /// This function makes a vertical line of size N starting from self.
    /// Example
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let line = point.make_vertical_line(5);
    /// line.iter().enumerate().for_each(|v|assert_eq!(v.1.y,v.0));
    /// ```
    pub fn make_vertical_line(self, number: usize) -> Vec<Point> {
        let mut line: Vec<Point> = Vec::new();
        for y in 0..=number {
            line.push(Point {
                y: self.y + y,
                x: self.x,
            });
        }
        line
    }
    pub fn make_horizontal_line(self, number: usize) -> Vec<Point> {
        let mut line: Vec<Point> = Vec::new();
        for x in 0..=number {
            line.push(Point {
                x: self.x + x,
                y: self.y,
            });
        }
        line
    }
}
impl<Q> From<PointWithItem<Q>> for Point {
    fn from(point: PointWithItem<Q>) -> Self {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}
impl<Q> From<&PointWithItem<Q>> for Point {
    fn from(point: &PointWithItem<Q>) -> Self {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}
impl From<(usize, usize)> for Point {
    fn from(point: (usize, usize)) -> Self {
        Self::new(point)
    }
}
impl From<&(usize, usize)> for Point {
    fn from(point: &(usize, usize)) -> Self {
        Self::new(*point)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
