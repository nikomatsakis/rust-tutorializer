#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    fn distance(&self, p: Point) -> f32 {
        (p.x - self.x).abs() + (p.y - self.y).abs()
    }
}

const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
};

#[test]
fn distance1() {
    let p1 = Point::new(1.0, 1.0);
    assert_eq!(ORIGIN.distance(p1), 1.0_f32.sqrt());
}

