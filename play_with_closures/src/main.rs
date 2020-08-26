use std::thread;
use std::time::Duration;

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
  // let simulated_user_specified_value = 18;
  // let simulated_random_number = 10;
  // generate_workout(simulated_user_specified_value, simulated_random_number);

  let v1 = vec![1, 2, 3, 4];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("Got: {}", val);
  }

  let v1: Vec<i32> = vec![1, 2, 3];

  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  println!("{:?}", v2);
}

// fn generate_workout(intensity: u32, random_number: u32) {
//   let mut expensive_result = Cacher::new(|num| {
//     println!("Doing some expensive calculation and generating your workout plan");
//     thread::sleep(Duration::from_secs(3));
//     num
//   });

//   if intensity < 25 {
//     let result = expensive_result.value(intensity);
//     println!("Today do {} pushups", result);
//     println!("After that do {} situps", result);
//   } else {
//     if random_number == 3 {
//       println!("Take a break today! Remember to stay hydrated");
//     } else {
//       println!(
//         "Today do cardio for {} minutes",
//         expensive_result.value(intensity)
//       );
//     }
//   }
// }

// #[test]
// fn call_with_different_values() {
//   let mut c = Cacher::new(|a| a);
//   let v1 = c.value(1);
//   let v2 = c.value(2);
//   assert_eq!(v2, 2);
// }

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();

  assert_eq!(total, 6);
}
