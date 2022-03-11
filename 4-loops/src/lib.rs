pub fn while_loop() -> i32 {
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    number
}

pub fn for_loop() -> i32 {
    let a = [1, 2, 3, 4, 5];

    let mut sum = 0;
    for e in a {
        sum += e;
    }
    sum
}

pub fn for_loop_range() -> i32 {
    let mut sum = 0;
    for number in 1..=5 {
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn while_loop_test() {
        let result = while_loop();
        assert_eq!(result, 0);
    }

    #[test]
    fn for_loop_test() {
        let result = for_loop();
        assert_eq!(result, 15);
    }

    #[test]
    fn for_loop_range_test() {
        let result = for_loop_range();
        assert_eq!(result, 15);
    }
}
