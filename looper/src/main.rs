fn main() {
    println!("Hello, world!");
    loop_10();
    loop_for(5);
    array_loop();
    nested_loop();
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
        if i % 2 == 0 {
            v.push(i);
        }
    }

    for n in v {
        println!("{}", n);
    }
}


fn nested_loop(){
  let v = vec![4,5,6,22,45,345];

  'outer: for i in 0..10 {
    // here we have to iterate over pointer to v
    //  as when we use v , we lose access to v
    for n in &v {
      if i + n == 11 {
        break 'outer;
      }
      println!("{}", n);
    }
  }
}