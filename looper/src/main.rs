fn main() {
    println!("Hello, world!");
    loop_10();
    loop_for(5);
    array_loop();
}

fn loop_10() {
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

fn array_loop() {
    let mut v = Vec::new();
    for i in 0..90 {
        v.push(i)
    }

    for n in v {
        println!("{}", n)
    }
}
