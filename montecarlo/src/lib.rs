/// A point struct that consists of an x and y coordinate.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Returns a Point with x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - A f64 that represents the x coordinate given to the line
    /// * 'y' - A f64 that represents the y coordinate given to the line
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }
}

/// A canvas struct that consists of the length and width of the canvas as well as points
pub struct Canvas {
    pub length: u32,
    pub width: u32,
    pub points: Vec<Point>,
}

impl Canvas {
    /// Returns a Canvas struct with length and width.
    /// 
    /// The value that is passed in that is larger will always be the length.
    ///
    /// # Arguments
    ///
    /// * `length` - A u16 that represents the length (may be flipped with width)
    /// * 'width' - A u16 that represents the width (may be flipped with length)
    pub fn new(length: u32, width: u32) -> Canvas {
        if length > width {
            Canvas {
                length: length,
                width: width,
                points: Vec::new(),
            }
        }
        else {
            Canvas {
                length: width,
                width: length,
                points: Vec::new(),
            }
        }
    }
}

/// A line struct that consists of two points, a and b
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    /// Returns a Line struct with length and width.
    ///
    /// # Arguments
    ///
    /// * `a` - A Point that is one end of the line
    /// * 'b' - A Point that is one end of the line
    pub fn new(a: Point, b: Point) -> Line {
        Line {
            a,
            b,
        }
    }
}

