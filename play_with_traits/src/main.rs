use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    x:i32,
    y:i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other:Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let x1 = Point{x: 3, y: 4};
    let x2 = Point{x: 3, y: 6};

    let result_point = x1 + x2;

    println!("{:?}", result_point)
}
