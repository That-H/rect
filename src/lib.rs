//! Library containing a rectangle type.

use point::Point;

/// A rectangle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    /// Largest y co-ord of the rect.
    pub top: i32,
    /// Farthest left x co-ord of the rect.
    pub left: i32,
    /// Width of the rect in tiles.
    pub wid: i32,
    /// Height of the rect in tiles.
    pub hgt: i32,
}

impl Rect {
    /// Create a new rectangle.
    pub fn new(left: i32, top: i32, wid: i32, hgt: i32) -> Self {
        Self {
            top,
            left,
            wid,
            hgt,
        }
    }

    /// Rightmost x co-ord of the rect.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect = Rect::new(1, 1, 4, 3);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
    /// //
    /// //  +--+
    /// // O|  |
    /// //  +--+
    /// // 
	///
    /// assert_eq!(rect.right(), 4);
    /// ```
    pub fn right(&self) -> i32 {
        self.left + self.wid - 1
    }

    /// Lowest y co-ord of the rect.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect = Rect::new(1, 1, 4, 3);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
    /// //
    /// //  +--+
    /// // O|  |
    /// //  +--+
	/// // 
    ///
    /// assert_eq!(rect.bottom(), -1);
    /// ```
    pub fn bottom(&self) -> i32 {
        self.top - self.hgt + 1
    }

    /// Returns true if the rect overlaps other.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect1 = Rect::new(0, 7, 4, 3);
    /// let rect2 = Rect::new(3, 6, 5, 5);
    /// let rect3 = Rect::new(10, 2, 3, 3);
    ///
    /// // The above rectangles, below:
	/// // '!' represents where an overlap occurs is.
	/// // 'O' is the origin.
    /// //
    /// // +--+
    /// // |1 !---+
    /// // +--!   |
	/// //    | 2 |
	/// //    |   |
    /// //    +---+  +-+
	/// //           |3|
	/// // O         +-+
	///
    /// assert!(rect1.overlaps(&rect2));
	/// assert!(rect2.overlaps(&rect1));
	/// assert!(!rect3.overlaps(&rect1));
	/// assert!(!rect3.overlaps(&rect2));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        self.left <= other.right()
            && self.right() >= other.left
            && self.top >= other.bottom()
            && self.bottom() <= other.top
    }

    /// Returns the top left corner as a point.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect = Rect::new(2, 1, 4, 3);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
    /// //
    /// // Corner in question
	/// //   |
	/// //   v
    /// //   +--+
    /// //   |  |
    /// //   +--+
	/// // O 
    ///
    /// assert_eq!(rect.top_left(), Point::new(2, 1));
    /// ```
    pub fn top_left(&self) -> Point {
        Point::new(self.left, self.top)
    }

    /// Returns each point on the edge of the rectangle.
    pub fn edges(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for x in self.left..=self.right() {
            points.push(Point::new(x, self.top));
            points.push(Point::new(x, self.bottom()));
        }

        for y in self.bottom() + 1..=self.top - 1 {
            points.push(Point::new(self.left, y));
            points.push(Point::new(self.right(), y));
        }

        points
    }

    /// Returns each corner of the rect.
	///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let mut rect = Rect::new(1, 1, 3, 5);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
    /// //
    /// //  +-+
    /// // O| |
    /// //  | |
    /// //  | |
    /// //  +-+
	/// // 
    ///
    /// let expected = vec![Point::new(1, 1), Point::new(3, 1), Point::new(1, -3), Point::new(3, -3)];  
    ///
    /// assert_eq!(rect.corners(), expected);
    /// ```
    pub fn corners(&self) -> Vec<Point> {
        let left = self.left;
        let right = self.right();
        let top = self.top;
        let bottom = self.bottom();

        vec![
            Point::new(left, top),
            Point::new(right, top),
            Point::new(left, bottom),
            Point::new(right, bottom),
        ]
    }

    /// Increases the size of the rectangle in the given direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let mut rect = Rect::new(1, 1, 3, 5);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
    /// //
    /// //  +-+
    /// // O| |
    /// //  | |
    /// //  | |
    /// //  +-+
    /// // 
	///
    /// let transformed = Rect::new(1, 1, 4, 5);
	/// rect.expand(Point::new(1, 0));
    ///
    /// assert_eq!(rect, transformed);
	///
    /// // After transformation:
    /// //
    /// //  +--+
    /// //  |  |
    /// //  |  |
    /// //  |  |
    /// //  +--+	
    /// ```
    pub fn expand(&mut self, dir: Point) {
        self.wid += dir.x.abs();

        if dir.x < 0 {
            self.left += dir.x;
        }

        self.hgt += dir.y.abs();

        if dir.y > 0 {
            self.top += dir.y;
        }
    }

    /// Checks whether the given position is within or on the rectangle's boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect = Rect::new(0, 0, 3, 5);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
	/// //
    /// // O-+
    /// // | |
    /// // | |
    /// // | |
    /// // +-+
    ///
    /// assert!(rect.contains(Point::new(1, -3)));
    /// assert!(rect.contains(Point::new(1, 0)));
    /// assert!(!rect.contains(Point::new(-1, 0)));
    /// ```
    pub fn contains(&self, pos: Point) -> bool {
        self.left <= pos.x && self.right() >= pos.x && self.top >= pos.y && self.bottom() <= pos.y
    }

    /// Return an iterator over all positions contained
    /// within the rect, including the edges.
    #[inline]
    pub fn cells(&self) -> InteriorIter {
        InteriorIter::from(*self)
    }

    /// Return all positions contained within the rect, excluding the edges.
    pub fn inner_cells(&self) -> InteriorIter {
        InteriorIter::from(Rect::new(self.left + 1, self.top - 1, self.wid - 2, self.hgt - 2))
    }

    /// Returns the area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let rect = Rect::new(0, 5, 3, 5);
    ///
    /// // The above rectangle, below:
	/// // 'O' is the origin.
	/// //
    /// // +-+
    /// // | |
    /// // | |
    /// // | |
    /// // O-+
    ///
    /// assert_eq!(rect.area(), 15);
    /// ```
    pub fn area(&self) -> u32 {
        (self.wid * self.hgt).unsigned_abs()
    }

    /// Relocates the rect's top left corner to the given position.
    pub fn move_to(&mut self, pos: Point) {
        self.left = pos.x;
        self.top = pos.y;
    }

    /// Centres the rect on the given position. When it is not possible to centre
	/// exactly on the provided co-ordinates, the new centre will be to the right of
	/// and/or below the true centre.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    /// use rect::Rect;
    ///
    /// let mut rect = Rect::new(0, 4, 4, 4);
	/// rect.centre_on(Point::new(4, 4));
    ///
    /// // The above rectangle (before centring), below:
	/// // 'C' is where the centre would be placed.
	/// //     C
    /// // +--+
    /// // |  |
    /// // |  |
    /// // O--+
    /// //
	/// // After centring:
	/// //              
	/// //   +--+          
	/// //   |  |          
	/// //   | C|        
	/// //   +--+      
	/// //         
	/// //        
	/// // O   
	/// //
	/// 
    /// assert_eq!(rect.top_left(), Point::new(2, 6));
    /// ```
    pub fn centre_on(&mut self, centre: Point) {
        self.move_to(centre + Point::new(-self.wid / 2, self.hgt / 2));
    }
}

/// An iterator over the cells inside a rect.
/// Iterates top to bottom, left to right.
#[derive(Clone, Debug)]
pub struct InteriorIter {
    cur_pos: Point,
    rect: Rect,
    end: bool,
}

impl Iterator for InteriorIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let ret = self.cur_pos;

        let mut new_x = self.cur_pos.x + 1;

        // Check if end of row.
        if new_x > self.rect.right() {
            new_x = self.rect.left;
            self.cur_pos.y -= 1;

            // Check if end of rectangle.
            if self.cur_pos.y < self.rect.bottom() {
                self.end = true;
            } else {
                self.cur_pos.x = new_x;
            }
        }

        self.cur_pos.x = new_x;

        Some(ret)
    }
}

impl From<Rect> for InteriorIter {
    fn from(val: Rect) -> Self {
        Self {
            cur_pos: val.top_left(),
            rect: val,
            end: false,
        }
    }
}

#[cfg(test)]
mod unittests {
    use super::*;

    #[test]
    fn cells_test() {
        let test_rect = Rect::new(1, 2, 3, 4);
        let mut expected = Vec::new();

        for y in (test_rect.bottom()..=test_rect.top).rev() {
            for x in test_rect.left..=test_rect.right() {
                expected.push(Point::new(x, y));
            }
        }

        assert_eq!(expected, test_rect.cells().collect::<Vec<Point>>());
    }
}
