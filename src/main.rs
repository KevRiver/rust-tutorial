use std::fmt::{self, Formatter, Display};

struct Point{
    x: f32,
    y: f32,
}

impl Display for Point{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

struct Rect{
    p0: Point,
    p1: Point,
}

impl Display for Rect{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "p0: {}\np1: {}", self.p0, self.p1)
    }
}

fn main() {
    let point1: Point = Point{ x: 0.3, y: 0.4};
    let point2: Point = Point{ x: 0.1, y: 0.3};
    println!("{}", point1);
    println!("{}", point2);

    let r: Rect = Rect{p0: point1 , p1: point2 };
    println!("{}", r);
}
