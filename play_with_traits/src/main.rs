use rand::Rng;
use std::ops::Add;
extern crate rand;

#[derive(Debug, Clone, Copy)]
pub struct Point {
  x: i32,
  y: i32,
}

pub trait Random {
  fn random() -> Point;
}

impl Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> Self::Output {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Random for Point {
  fn random() -> Self {
    let mut tr = rand::thread_rng();
    Point {
      x: tr.gen(),
      y: tr.gen(),
    }
  }
}

// News article

pub trait Summary {
  fn summarize(&self) -> String {
    format!("(Read More...), {}", self.summarize_author())
  }
  fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

fn main() {
  println!("Hello, world!");
  let x1 = Point { x: 3, y: 4 };
  let x2 = Point { x: 3, y: 6 };

  let result_point = x1 + x2;

  println!("{:?}", result_point);

  let d = Point::random();
  println!("d {:?}", d);

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
  notify(&tweet);
}
