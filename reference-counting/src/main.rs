use std::rc::Rc; // smart pointer that allows multiple ownership

#[derive(Debug)]
struct Alien {
    name: String,
    n_tentacles: u32,
}

#[derive(Debug)]
struct Tentacle {
    alien: Rc<Alien>,
    length: u32,
}

fn main() {
    println!("Hello, world!");

    let alien: Rc<Alien> = Rc::new(Alien {
        name: String::from("Xenomorph"),
        n_tentacles: 8,
    });

    println!("Address of alien {:p}", alien);

    let mut tentacles: Vec<Tentacle> = Vec::new();

    for i in 0..alien.n_tentacles {
        let tentacle = Tentacle {
            alien: alien.clone(), // copy the Rc pointer, not the Alien struct
            length: i,
        };
        tentacles.push(tentacle);
    }

    // check the addresses of tentacle's alien
    for tentacle in tentacles {
        println!("{:p}", tentacle.alien);
    }
}
