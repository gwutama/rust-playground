use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let vars: Vec<(String, String)> = env::vars().collect();
    println!("Environment variables: {:?}", vars);
}
