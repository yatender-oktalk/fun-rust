
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

    let s3 = s2.clone();
    println!("{}", s3);


    let mut s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = take_and_gives_back(s5);
    let (s7, _len) = calculate_length(s6);
    let len = calculate_length_reference(&s7);
    println!("{}", len);


    change(&mut s4);
    println!("{}", s4);

    let s9 = dangle();
    println!("{}", s9);

    let mut s10 = String::from("yo man how are you?");
    let length = first_word(&s10);
    println!("{}", length);

    s10.clear();
    let s11 = gives_ownership();
    let hello = &s11[0..=4];
    let brother = &s11[6..14];
    println!("{}", hello);
    println!("{}", brother);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello brother!");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
   some_string.push_str(", world");
}

fn dangle() -> String {
    let dangle_str = String::from("Hello bros");
    dangle_str
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

