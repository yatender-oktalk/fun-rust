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

fn main() {
    let number_list = vec![34, 50, 67, 89, 11, 23];
    let result = largest_i32(&number_list);
    println!("the largest number is {}", result);

    let char_list = vec!['y', 'a', 'h', 'P'];

    let result = largest_char(&char_list);
    println!("the largest char is {}", result);
}
