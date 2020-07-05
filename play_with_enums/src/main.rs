
#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge,
}

#[derive(Debug)]
pub struct Bed{
    size:i32,
    count:u32
}

fn main() {
    use self::Room::*;
    let t = Kitchen(4);
    let t2 = Bedroom(Bed{size: 10, count: 3});
    println!("Hello, world! {:?}", t);
    println!("Hello, world! {:?}", t2);

    let v = match t {
        Kitchen(n) =>  n,
        _ => 0,
    } + 10;

    println!("{}", v)
}
