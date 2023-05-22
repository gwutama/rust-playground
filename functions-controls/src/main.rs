fn main() {
    println!("Hello, world!");

    let dead = false;
    let health = 100;

    if dead {
        println!("You are dead!");
    } else if health < 50 {
        println!("You are almost dead!");
    } else {
        println!("You are alive!");
    }

    let is_alive = if !dead { true } else { false };
    println!("Is alive: {}", is_alive);

    let max_power = 100;
    let mut power = 1;

    while power < max_power {
        print!("{} ", power);
        power += 1;
    }

    println!();

    greet("John");

    let magic = magic_number();
    println!("Magic number: {}", magic);
    println!("Sexy number: {}", sexy_number());
}

/// Greets the given name.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

/// Returns the magic number.
///
/// # Examples
/// ## Example 1
/// Can ~~we~~ __format__ *this* **text**?
fn magic_number() -> i32 {
    42
}

/// Returns the sexy number.
///
/// # Examples
/// sexy_number();
///
/// Why?
/// - it is sexy
/// - it is a number
fn sexy_number() -> i32 {
    return 69;
}
