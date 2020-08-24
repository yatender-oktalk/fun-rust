#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greetings(name: &str) -> String {
    format!("hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_should_contain_name() {
        let result = greetings("yatender");
        assert!(result.contains("yatender"));
    }

    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 4,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn greater_than_100() {
        Guess::new(20);
    }
}
