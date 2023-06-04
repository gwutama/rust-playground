fn main() {
    println!("Hello, world!");
    let x = 0;
    let y = 0;

    let result = div_safe(x, y);
    println!("{} / {} = {}", x, y, result.unwrap_or(0));

    assert!(y != 0, "divison by zero (assert)");

    let result2 = div(x, y);
    println!("{} / {} = {}", x, y, result2);
}

fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("divison by zero (panic)");
    }

    return x / y;
}

fn div_safe(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        return None;
    }

    return Some(x / y);
}
