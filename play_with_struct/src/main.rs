#[derive(Debug)]
struct User{
    name: String,
    age: i32,
    height: i32,
    shoe_size: i8,
}

fn main() {
    let u = User{
        name: String::from("matt"),
        age: 32,
        height: 160,
        shoe_size: 8,
    };

    println!("user is {:?}", u)
}
