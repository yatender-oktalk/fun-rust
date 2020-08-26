#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn filter_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 8,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 9,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 9,
        style: String::from("boot"),
      },
    ];

    let my_size_shoes = shoes_in_my_size(shoes, 9);
    assert_eq!(
      my_size_shoes,
      vec![
        Shoe {
          size: 9,
          style: String::from("sneaker"),
        },
        Shoe {
          size: 9,
          style: String::from("boot"),
        },
      ]
    )
  }
}
