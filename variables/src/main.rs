
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

}
