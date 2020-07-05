use std::io;
use std::string::String;

fn main() {
    println!("Enter your name!");
    say_hello();
}


fn say_hello() -> String {
    // if name's first char starts with the 'w' then say hello 
    // otherwise say bye bye

    // Enter your name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Should be a string!");
    println!("entered name {}", &name);
}
