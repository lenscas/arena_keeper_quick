use super::Point;

pub struct PointWithItem<T> {
    pub x: usize,
    pub y: usize,
    pub item: T,
}
impl<T> PointWithItem<T> {
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
    pub fn make_line<Q>(self, other: Point, map: impl Fn(&Point) -> Q) -> Vec<PointWithItem<Q>> {
        let point: Point = self.into();
        point
            .make_line(other)
            .iter()
            .map(|v| PointWithItem {
                x: v.x,
                y: v.y,
                item: map(v),
            })
            .collect()
    }
}
