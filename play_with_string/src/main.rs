fn main() {
    let s = String::from("Hello 大学生");

    for c in s.chars() {
        println!("{}",c);
    }

    for c in s.bytes() {
        println!("{}",c)
    }
}
