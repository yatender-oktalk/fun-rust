
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

    // slices
    let mut s10 = String::from("yo man how are you?");
    let length = first_word(&s10);
    println!("{}", length);

    s10.clear();

    let s11 = gives_ownership();
    let f_word = first_word_slice(&s11);
    println!("sparta {}", f_word);

    let f2_word = second_word_slice(&s11);
    println!("second {}", f2_word);

    let hello = &s11[0..=4];
    let brother = &s11[6..14];
    let brot = &s11[..14];
    let bro = &s11[0..];
    let l = s11.len();
    let bro_full = &s11[..];
    let bro_full_l = &s11[0..l];
    println!("{}", hello);
    println!("{}", brother);
    println!("{}", bro);
    println!("{}", brot);
    println!("{}", bro_full);
    println!("{}", bro_full_l);
    println!("brother legnth {}", l);

    // other slices
    let a = [1,2,3,4,5,5];
    // let slice = &a[..2];
    println!("{}", a[2]);


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

// this method gives back the reference of the substring
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}
