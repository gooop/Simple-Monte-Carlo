/// A point struct consists of an x and y coordinate.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Returns a Point with x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 that represents the x coordinate given to the line
    /// * 'y' - A f32 that represents the y coordinate given to the line
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }
}

/// A canvas struct consists of the length and width of the canvas as well as points and other shapes
pub struct Canvas {
    pub length: u32,
    pub width: u32,
    pub points: Vec<Point>,
    pub circles: Vec<Circle>,
}

impl Canvas {
    /// Returns a Canvas struct
    /// 
    /// The value that is passed in that is larger will always be the length.
    ///
    /// # Arguments
    ///
    /// * `length` - A u32 that represents the length (may be flipped with width)
    /// * 'width' - A u32 that represents the width (may be flipped with length)
    pub fn new(length: u32, width: u32) -> Canvas {
        if length > width {
            Canvas {
                length: length,
                width: width,
                points: Vec::new(),
                circles: Vec::new(),
            }
        }
        else {
            Canvas {
                length: width,
                width: length,
                points: Vec::new(),
                circles: Vec::new(),
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
    /// Returns a Line struct
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

/// a circle struct consists of a point, a, and a radius, r
pub struct Circle {
    pub a: Point,
    pub r: f32,
}

impl Circle {
    /// Returns a Circle struct
    ///
    /// # Arguments
    ///
    /// * `a` - A Point in the center of the Circle
    /// * 'r' - The radius of the Circle
    pub fn new(a: Point, r: f32) -> Circle {
        Circle {
            a,
            r,
        }
    }
}
