struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    phone: String,
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    println!("Hello, world!");

    // simple invocation
    let user1 = User {
        email: String::from("yatender.nitk@outlook.com"),
        username: String::from("yatender.nitk"),
        phone: String::from("8105139417"),
        active: true,
        sign_in_count: 1
    };

    // updating using existing values
    let u2_email = String::from("another@example.com");
    let u2_phone = String::from("9090909090");
    let u2_username = String::from("anotherusername567");
    let user2 = build_user(u2_email, u2_phone, u2_username);

    // nicer & simple way
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        phone: String::from("9090909090"),
        ..user1
    };

    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{}", user3.email);


    let black = Color(60, 3, 5);
    let origin = Point(50, 30, 10);
    println!("{}", black.0);

    let area = area(30, 50);
    println!("{}", area);

    let dim_tuple = (30, 60);
    let tuple_area = area_tuple(dim_tuple);
    println!("{}", area);

    let rect = Rectangle {width: 30, height: 50};
    println!("area {}", area_struct(&rect));
    //to print the whole struct type enable derive debug and add {:?} format
    // & pass any struct type
    println!("area {:?}", rect);
}

fn build_user(email: String, phone: String, username: String) -> User {
    User {
        email: email,
        username: username,
        phone: phone,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, phone: String, username: String) -> User {
    User {
        email,
        username,
        phone,
        active: true,
        sign_in_count: 1,
    }
}


fn area_tuple(dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(w : u32,h: u32) -> u32 {
    w * h
}

fn area_struct(dims: &Rectangle) -> u32 {
    dims.width * dims.height
}