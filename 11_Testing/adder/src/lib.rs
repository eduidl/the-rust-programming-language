#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", &name[0..(name.len() - 1)])
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guesss value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guesss value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger  = Rectangle { length: 8, height: 7 };
        let smaller = Rectangle { length: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger  = Rectangle { length: 8, height: 7 };
        let smaller = Rectangle { length: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
             "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(ecpected = "Guess value must be less than or equal to 100")]
    fn greater_tha_100() {
        Guess::new(200);
    }
}
