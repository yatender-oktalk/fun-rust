struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("{}", integer.x);
    println!("{}", integer.y);
    let float = Point { x: 1.3, y: 3.2 };
    println!("{}", float.y);
    println!("{}", float.y);
    println!("Hello, world!");
}
