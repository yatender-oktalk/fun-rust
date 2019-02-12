
fn main() {
    let mut x: u32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let mut s = String::from("hello how are you");
    s.push_str(", world!");
    s.push('w');
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let s3 = s2.clone();
    println!("{}", s3);


    let _s4 = gives_ownership();
    let s5 = String::from("hello");
    let _s6 = take_and_gives_back(s5);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}
