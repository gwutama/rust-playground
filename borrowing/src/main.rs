#[derive(Debug)]
struct Alien {
    name: String,
    planet: String,
}

fn main() {
    println!("Hello, world!");

    let alien1 = Alien {
        name: "ET".to_string(),
        planet: "Mars".to_string(),
    };
    let mut alien2 = alien1; // ownership moved from alien1 to alien2
    // alien1.name = "Yoda".to_string(); // not possible anymore, ownership was moved to alien2

    let mut alien3 = &mut alien2; // alien3 is a reference to alien2 (borrowed)
    alien3.name = "Yoda".to_string(); // possible because alien2 is borrowed
    println!("alien3: {:?}", alien3);

    // alien2.planet = "Earth".to_string(); // COMPILE CHECK. not possible because alien3 is still doing something with it

    alien3.name = "ET".to_string(); // possible because alien2 is borrowed
    println!("alien3: {:?}", alien3);

    alien2.planet = "Earth".to_string(); // possible because alien3 doesnt do anything to it anymore

    // or, for clarity, we can do this in a block:
    {
        let alien4 = &mut alien2;
        alien4.planet = "Earth".to_string();
        println!("alien4: {:?}", alien4);
    } // alien4 is dropped here

    alien2.planet = "Earth".to_string(); // possible because alien4 doesnt do anything to it anymore
}
