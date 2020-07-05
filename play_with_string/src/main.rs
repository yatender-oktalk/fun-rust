fn main() {
    let s = String::from("Hello 大学生");

    println!("{}", s.len());

    for (i, c) in s.char_indices() {
        println!("{}, {}", c, i);
    }

    for c in s.bytes() {
        println!("{}", c)
    }

    println!("{}", count_l(&s));
}

fn count_l(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    res
}
