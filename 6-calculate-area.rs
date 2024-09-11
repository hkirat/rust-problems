enum Shape {
    Rectangle(f64, f64), // width, height
    Circle(f64),         // radius
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let circle = Shape::Circle(1.0);
    calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}

