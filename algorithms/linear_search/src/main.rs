fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];
  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 67, 89, 11, 23];
  let result = largest_i32(&number_list);
  println!("the largest number is {}", result);

  let char_list = vec!['y', 'a', 'h', 'P'];

  let result = largest_char(&char_list);
  println!("the largest char is {}", result);

  let number_list_new = vec![34, 40, 45, 67, 33, 88];
  let result = largest(&number_list_new);
  println!("{}", result);

  let s1 = String::from("abcdefgh");
  let s2 = String::from("abcd");
  println!("{}", longest(&s1, &s2))
}
