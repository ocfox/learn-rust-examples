#[derive(Debug)]
// 带有两个字段（field）的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let point1 = Point { x: 3.0, y: 5.0 };
    println!("{:?}", point1);
    let rec1 = square(point1, 5.0);
    let area = rect_area(rec1);
    println!("The area of Rectangle is {}", area);
}

fn square(Point { x, y }: Point, leng: f32) -> Rectangle {
    Rectangle {
        p1: Point { x, y },
        p2: Point {
            x: x + leng,
            y: y + leng,
        },
    }
}

fn rect_area(Rectangle { p1, p2 }: Rectangle) -> f32 {
    (p2.x - p1.x) * (p2.y - p1.y)
}
