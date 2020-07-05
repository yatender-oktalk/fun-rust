#[derive(Debug)]
struct User{
    name: String,
    age: i32,
    height: i32,
    shoe_size: i8,
}

impl User{
    fn simple_string(&self) -> String {
        format!("{} - {} - {}cm - shoe_size: {}", self.name, self.age, self.height, self.shoe_size)
    }

    fn grow(&mut self, h: i32) {
      self.height += h;
    }

    fn die(self) {
      println!("Dead {}", self.simple_string());
    }
}

fn main() {
    let mut u = User{
        name: String::from("matt"),
        age: 32,
        height: 160,
        shoe_size: 8,
    };

    println!("user is {:?}", u.simple_string());
    u.grow(20);
    u.grow(20);
    println!("user is {:?}", u.simple_string());
    u.die()
}
