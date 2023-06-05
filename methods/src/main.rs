struct Alien {
    name: String,
    n_tentacles: u32,
    n_eyes: u32,
    n_arms: u32,
    health: u32,
    damage: u32,
}

impl Alien {
    fn new(name: String, n_tentacles: u32, n_eyes: u32, n_arms: u32, mut health: u32, damage: u32) -> Alien {
        if health > 100 {
            health = 100;
        }
        Alien {
            name,
            n_tentacles,
            n_eyes,
            n_arms,
            health,
            damage,
        }
    }

    fn attack(&mut self) {
        self.health -= 10;
        println!("{} was attacked and now has {} health", self.name, self.health);
    }

    // no overloading possible
    // fn attack(&mut self, damage: u32) {
    fn attack_with_damage(&mut self, damage: u32) {
        self.health -= damage;
        println!("{} was attacked and now has {} health", self.name, self.health);
    }
}

fn main() {
    println!("Hello, world!");

    let mut bork = Alien::new(String::from("Bork"), 8, 4, 2, 200, 100);
    println!("{} has {} health", bork.name, bork.health);
    bork.attack();
    bork.attack_with_damage(20);
}
