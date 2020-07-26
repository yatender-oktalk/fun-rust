//!

#[derive(Debug)]
struct Human {
    name: String,
    age: u16,
}

impl Human {
    pub fn new(n: String, a: u16) -> Self {
        Self { name: n, age: a }
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    push_in_list(&mut v, 23);
    push_in_list(&mut v, 33);
    push_in_list(&mut v, 53);
    println!("{:?}", v);
    let mut v_human: Vec<Human> = Vec::new();
    let human_1 = Human::new(String::from("Yatender"), 10);
    let human_2 = Human::new(String::from("milky silky"), 3);
    push_into_vec(&mut v_human, human_1);
    push_into_vec(&mut v_human, human_2);
    println!("{:?}", v_human);
}

fn push_in_list(v: &mut Vec<i32>, number: i32) -> &Vec<i32> {
    v.push(number);
    v
}

fn push_into_vec(v: &mut Vec<Human>, human: Human) -> &Vec<Human> {
    v.push(human);
    v
}
