fn main() {
    let mut v: Vec<i32> = Vec::new();
    push_in_list(&mut v, 23);
    push_in_list(&mut v, 33);
    push_in_list(&mut v, 53);
    println!("{:?}", v);
}

fn push_in_list(v: &mut Vec<i32>, number: i32) -> Vec<i32> {
    v.push(number);
    v.to_vec()
}
