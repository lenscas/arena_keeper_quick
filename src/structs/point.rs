/// A simple single coordinate
pub struct Point {
    pub x : usize,
    pub y : usize
}

impl Point {
    /// a way to add an item to a coordinate.
    /// ```
    /// # use arena::structs::point::Point;
    /// # let point = Point {x : 0, y : 0};
    /// assert_eq!(point.add_item(10).item,10);
    /// ```
    pub fn add_item<T>(&self, item : T) -> PointWithItem<T> {
        PointWithItem {
            x : self.x,
            y : self.y,
            item
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
    /// Diagonal line:
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:3,y:3};
    /// let line = point.make_line(point2);
    /// let mut x : usize = 0;
    /// let mut y : usize = 0;
    /// for coordinate in 0 ..= 6 {
    ///     assert_eq!(line[coordinate].x,x);
    ///     assert_eq!(line[coordinate].y,y);
    ///     if coordinate % 2 == 0 {
    ///         y += 1;
    ///     } else {
    ///         x += 1;
    ///     }
    ///     
    /// }
    /// ```
    pub fn make_line(self, other : Point) -> Vec<Point> {
        if other.x != self.x && self.y == other.y {
            if self.x > other.x {
                let diff = 1 + self.x - other.x;
                other.make_horizontal_line(diff)
            } else {
                let diff = 1 + other.x - self.x;
                self.make_horizontal_line(diff)
            }
        } else if other.y != self.y && other.x == self.x {
            if self.y > other.y {
                let diff = 1 + self.y - other.y;
                other.make_vertical_line(diff)
            } else {
                let diff = 1 + other.y - self.y;
                self.make_vertical_line(diff)
            }
        } else {
            self.make_diagonal_line(other)
        }
        
    }
    /// This function is used to get all points in a diagonal line.
    /// Warning: This function breaks when asked to draw either a horizontal or vertical line
    /// Use make_line, make_vertical_line or make_horizontal_line instead
    /// #Examples:
    /// Working:
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:3,y:3};
    /// let line = point.make_line(point2);
    /// let mut x : usize = 0;
    /// let mut y : usize = 0;
    /// for coordinate in 0 ..= 6 {
    ///     assert_eq!(line[coordinate].x,x);
    ///     assert_eq!(line[coordinate].y,y);
    ///     if coordinate % 2 == 0 {
    ///         y += 1;
    ///     } else {
    ///         x += 1;
    ///     }
    ///     
    /// }
    /// ```
    /// Broken:
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:0,y:5};
    /// let line = point.make_diagonal_line(point2);
    /// assert_eq!(line.len(), 2);
    /// ```
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let point2 = Point {x:5,y:0};
    /// let line = point.make_diagonal_line(point2);
    /// assert_eq!(line.len(), 2);
    /// ```
    pub fn make_diagonal_line(self,other : Point) -> Vec<Point> {
        let mut coordinates = vec![];
        let dx:i32 = i32::abs(other.x as i32 - self.x as i32);
        let dy:i32 = i32::abs(other.y as i32 - self.y as i32);
        let sx:i32 = {
            if self.x < other.x {
                1
            } else {
                -1
            }
        };
        let sy:i32 ={
            if self.y < other.y {
                1
            } else {
                -1
            }
        };
        let mut error:i32 = dx - dy;
        let mut current_x:i32 = self.x as i32;
        let mut current_y:i32 = self.y as i32;
        coordinates.push(Point { x: current_x as usize, y: current_y as usize });
        while current_x != other.x as i32 && current_y != other.y as i32 {
            let error2:i32 = 2 * error;
            if error2 >= i32::abs(dy) {
                error -= dy;
                current_x += sx;
                coordinates.push(Point { x: current_x as usize, y: current_y as usize });
            } else if error2 <= i32::abs(dx) {
                error += dx;
                current_y += sy;
                coordinates.push(Point { x: current_x as usize, y: current_y as usize });
            }
        }
        coordinates.push(other);
        coordinates
    }
    /// This function makes a vertical line of size N starting from self.
    /// Example
    /// ```
    /// # use arena::structs::point::Point;
    /// let point = Point {x:0,y:0};
    /// let line = point.make_vertical_line(5);
    /// line.iter().enumerate().for_each(|v|assert_eq!(v.1.y,v.0));
    /// ```
    pub fn make_vertical_line(self,number : usize) -> Vec<Point> {
        let mut line : Vec<Point> = Vec::new();
        for y in 0..=number {
            line.push(Point{y : self.y + y, x : self.x});
        }
        line
    }
    pub fn make_horizontal_line(self, number : usize) -> Vec<Point> {
        let mut line : Vec<Point> = Vec::new();
        for x in 0..=number {
            line.push(Point{x : self.x + x, y : self.y});
        }
        line
    }
}
impl<Q> From<PointWithItem<Q>> for Point {
    fn from(point : PointWithItem<Q>) -> Self {
        Point {
            x : point.x,
            y : point.y
        }
    }
}

pub struct PointWithItem<T> {
    pub x : usize,
    pub y : usize,
    pub item : T
}
impl <T> PointWithItem<T> {
    /// A way to get all points between two points. Usefull to draw lines over the grid
    /// # Examples
    /// Horizontal line:
    /// ```
    /// # use arena::structs::point::PointWithItem;
    /// # use arena::structs::point::Point;
    /// let point = PointWithItem {x:0,y:0,item:"item"};
    /// let point2 = Point {x:3,y:0,};
    /// let line = point.make_line(point2,|v|v.x);
    /// for coordinate in 0 ..= 3 {
    ///     assert_eq!(line[coordinate].x , coordinate);
    ///     assert_eq!(line[coordinate].item , coordinate);
    /// }
    /// ```
    /// Vertical line:
    /// ```
    /// # use arena::structs::point::Point;
    /// # use arena::structs::point::PointWithItem;
    /// let point = PointWithItem {x:0,y:0,item:"item"};
    /// let point2 = Point {x:0,y:3};
    /// let line = point.make_line(point2,|v|v.y);
    /// for coordinate in 0 ..= 3 {
    ///     assert_eq!(line[coordinate].y , coordinate);
    ///     assert_eq!(line[coordinate].item , coordinate);
    /// }
    /// ```
    /// Diagonal line:
    /// ```
    /// # use arena::structs::point::Point;
    /// # use arena::structs::point::PointWithItem;
    /// let point = PointWithItem {x:0,y:0,item:0};
    /// let point2 = Point {x:3,y:3};
    /// let line = point.make_line(point2,|v|"item");
    /// let mut x : usize = 0;
    /// let mut y : usize = 0;
    /// for coordinate in 0 ..= 6 {
    ///     assert_eq!(line[coordinate].x,x);
    ///     assert_eq!(line[coordinate].y,y);
    ///     assert_eq!(line[coordinate].item,"item");
    ///     if coordinate % 2 == 0 {
    ///         y += 1;
    ///     } else {
    ///         x += 1;
    ///     }
    ///     
    /// }
    /// ```
    pub fn make_line<Q>(self, other : Point, map : impl Fn(&Point) -> Q) -> Vec<PointWithItem<Q>> {
        let point : Point = self.into();
        point.make_line(other).iter().map(|v| 
            PointWithItem {
                x : v.x,
                y : v.y,
                item : map(v)
            }
        ).collect()
    }
}