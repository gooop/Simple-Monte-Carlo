pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x,
            y,
        }
    }
}

pub struct Canvas {
    pub length: u16,
    pub width: u16,
}

