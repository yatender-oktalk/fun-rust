use std::collections::HashMap;
use std::env::args;

fn main() {
    println!("Hello, world!");

    let mut hm = HashMap::new();

    hm.insert(2, "hello");
    hm.insert(3, "world");

    let r = match hm.get(&3) {
        Some(v) => v,
        _ => "NOTHING",
    };

    println!("{}", r);
    
    
    let x = hm.get(&3).unwrap_or(&"No String");
    
    println!("{}", x);

    match get_arg(3) {
        Ok(v) => println!("ok - {}", v),
        Err(e) => println!("Error - {}", e),
    };
}


fn get_arg(n:usize) -> Result<String, String> {
    for (i, a) in args().enumerate() {
        if i ==n {
            return Ok(a)
        }
    }
    Err("Not enough Args".to_string())
}