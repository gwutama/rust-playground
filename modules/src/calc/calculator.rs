pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn div(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("Division by zero!");
    }

    return a / b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(3, 4), 12);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4.0, 2.0), 2.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero!")]
    fn test_div_division_by_zero() {
        assert_eq!(div(4.0, 0.0), 0.0);
    }
}