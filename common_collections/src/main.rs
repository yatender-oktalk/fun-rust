#![allow(unused_variables)]
    use std::collections::HashMap;


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3,4,5];
    println!("{:?}", v);


    //updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", &v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        print!("{} ", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = " bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    // let teams  = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // //zipping values
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);


    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}