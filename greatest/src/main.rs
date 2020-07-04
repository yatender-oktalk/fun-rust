fn main() {
    println!("Hello, world!");
    let response :i32  = highest(3,54, 64);
    let o :i32  = other(2,8);
    println!("{} is greatest", response);
    println!("{} is greatest", o);
}


fn highest(a: i32, b: u32, c: i8) -> i32 {
    let mut res = a;

    if b as i32 > res {
        res = b as i32
    }

    if c as i32 > res {
        res = c as i32
    }

    res
}

fn other(a: i32, b: i32) -> i32 {
    let c = a + b;
    let c = c % 4;
    let c = c / 2;
    c
}