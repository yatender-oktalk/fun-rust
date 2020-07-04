fn main() {
    println!("Hello, world!");
    loop10();
}

fn loop10() {
    let mut n = 0;
    loop {
        n += 1;
        println!("hello");
        if n >= 10 {
            return; 
        }
    }
}

