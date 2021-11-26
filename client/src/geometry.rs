struct Space {
    width: u32,
    height: u32,
    scroll_y: u32
}

struct Point {
    x: f64,
    y: f64,
    positioning: Positioning
}

impl Point {
    fn new_space
}

enum Positioning {
    Space,
    Frame,
    Sticky,
    Relative(Box<Point>)
}