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

    // Slices
    let slice = &vec2[1..3];
    println!("slice {:?}", slice);
}
