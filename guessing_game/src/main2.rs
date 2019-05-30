fn main() {
    let x = get_tuple();
    let number: u32 = 5;
    let n = is_greater(number);
    println!("{}", n);
    println!("{}, {}", x.1, x.0);
}

fn get_tuple() -> (f32, &'static str) {
    (5.0, "yatender")
}

fn is_greater(num: u32) -> &'static str {
    println!("{}", num);
     match num {
        Some(num) if num < 10 =>
        "greater than 10",
        _ =>
        "less than 10",
    }
}

