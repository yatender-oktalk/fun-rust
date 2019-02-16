struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    phone: String,
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
