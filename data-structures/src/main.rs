fn main() {
    println!("Hello, world!");

    // Strings
    let foo = "öÖäÄüÜß";
    println!("{}", foo);

    let mut str1 = String::new();
    println!("{}", str1);
    str1.push_str("Hello, ");
    println!("{}", str1);
    str1.push_str("world!");
    println!("{}", str1);

    if foo == "öÖäÄüÜß" {
        println!("foo is equal to öÖäÄüÜß");
    } else {
        println!("foo is not equal to öÖäÄüÜß");
    }

    // Arrays
    let aliens = ["Cherfer", "Gorble", "Nictor", "Zorble"];
    println!("The third alien is {}", aliens[2]);
    println!("{:?}", aliens);

    let mut arr: [i32; 3] = [1, 2, 3];
    arr[0] = 4;
    println!("{:?}", arr);
    println!("{}", arr.len());

    // Vectors
    let mut vec1: Vec<i32> = Vec::new();
    let vec2: Vec<i32> = vec![1, 2, 3];
    println!("vec1 {:?}", vec1);
    println!("vec2 {:?}", vec2);

    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("vec1 {:?}", vec1);
    let pop: Option<i32> = vec1.pop();
    println!("vec1 {:?}, pop {}", vec1, pop.unwrap_or_default());

    let chars: Vec<char> = foo.chars().collect();

    for c in chars.iter() {
        println!("{}", c);
    }

    // Slices
    let slice = &vec2[1..3];
    println!("slice {:?}", slice);

    // Tuples
    let thor = ("Thor", true, 3500);
    println!("{:?}", thor);

    println!("{}'s power is {}", thor.0, thor.2);
    let (name, _, power) = thor;
    println!("{}'s power is {}", name, power);

    // Structs
    let mut player1 = Player {
        name: String::from("Player_1"),
        health: 100,
        level: 1,
        score: 0,
    };
    player1.health = 90;
    println!("{} {} {} {}", player1.name, player1.health, player1.level, player1.score);

    // Enums
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let player_direction: Direction = Direction::Down;

    // Result
    let result_ok = get_result_ok();
    let result_error = get_result_error();
    println!("{:?} {:?}", result_ok, result_error);

    // Option
    let option_some = get_option_some();
    let option_none = get_option_none();
    println!("{:?} {:?}", option_some, option_none);
}

fn get_result_ok() -> Result<bool, String> {
    return Ok(true);
}

fn get_result_error() -> Result<bool, String> {
    return Err(String::from("Error"));
}

fn get_option_some() -> Option<String> {
    return Some(String::from("Hello"));
}

fn get_option_none() -> Option<String> {
    return None;
}

struct Player {
    name: String,
    health: i32,
    level: u8,
    score: u64,
}
