fn main() {
    println!("Hello, world!");
    loop10();
    loop_for(5);
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

fn loop_for(n: i32) {
    for i in 0..n {
        println!("{} yo", i);
    }
}
