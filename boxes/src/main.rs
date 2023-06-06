#[derive(Debug)]
struct Alien {
    name: String,
    planet: String,
}

fn main() {
    println!("Hello, world!");

    let mut alien1 = Box::new(Alien {
        name: String::from("Zoey"),
        planet: String::from("Earth"),
    });

    let mut alien2 = &mut alien1; // alien1 is borrowed by alien2
    alien2.name = String::from("Marcus");

    alien1.name = String::from("Zoey"); // compile check. ok because alien2 doesn't do anything to it anymore

    println!("Address of alien1: {:p}", &alien1);
    let alien3 = alien1; // alien1 is moved to alien3, but only the address, not the values
    println!("Address of alien3: {:p}", &alien3);

    let mut n = Box::new(42);
    println!("n = {}", n);
    *n = 10;
    println!("n = {}", n);

    let o = & mut *n; // or simply &n
    println!("o = {}", o);
    *o = 20; // compile check. not ok because o is immutable
    println!("o = {}", o);
    println!("n = {}", n);
}
