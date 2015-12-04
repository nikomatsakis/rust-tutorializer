use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

#[derive(Copy, Clone, Debug)]
enum Shape {
    /// Center and radius.
    Circle(Point, f32),

    Rectangle { corner1: Point, corner2: Point }
}

impl Shape {
    fn area(&self) -> f32 {
        // Goal: Compute the area for a shape.
        //
        // Tip: find out what kind of shape it is by matching on `*self`
        //
        // Question: Do you see why you would match on `*self`?
        // START SOLUTION
        match *self {
            Shape::Circle(_, radius) =>
                radius * radius * PI,
            Shape::Rectangle { corner1, corner2 } => {
                let width = (corner2.x - corner1.x).abs();
                let height = (corner2.y - corner1.y).abs();
                width * height
            }
        }
        // END SOLUTION
    }
}

#[test]
fn test1() {
    let shape = Shape::Circle(ORIGIN, 1.0);
    assert_eq!(shape.area(), PI);
}

#[test]
fn test2() {
    let shape = Shape::Rectangle {
        corner1: ORIGIN,
        corner2: Point::new(5.0, 6.0),
    };
    assert_eq!(shape.area(), 30.0);
}

#[test]
fn test3() {
    println!("WTF");
    let shape = Shape::Rectangle {
        corner1: ORIGIN,
        corner2: Point::new(-5.0, 6.0),
    };
    assert_eq!(shape.area(), 30.0);
}
