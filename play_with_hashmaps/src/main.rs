use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let score = vec![10, 20];
    let scores: HashMap<_, _> = teams.into_iter().zip(score.into_iter()).collect();
    println!("{:?}", scores);

    let mut map = HashMap::new();
    map.insert("hello", "world");
    map.insert("hello2", "world2");

    // println!("{:?}", map.get("hell3o"));
    println!("{:?}", map.entry("hello32").or_insert("90"));

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
