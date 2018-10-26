#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = rect;

    return ((y2 - y1) * (x2 - x1)).abs();
}

fn main() {
    let name = "Peter";
    let age = 10;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 0.3, y: 0.4 };
    println!("Access ({} {})", point.x, point.y);

    let new_point = Point { y: 0.1, ..point };
    println!("Access ({} {})", new_point.x, new_point.y);

    let rect = Rectangle {
        p1: Point { x: 1f32, y: 2f32 },
        p2: point,
    };

    println!("{:?}", rect);
    println!("Area {}", rect_area(rect));
}