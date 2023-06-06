use log::{info, warn};
use simple_logger::SimpleLogger;

mod printer;
mod calc;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Starting up!");
    printer::stdout("Hello, world!");
    // printer::stdout("Hello, world!"); // not possible. private function.

    let result1 = calc::add(1, 2);
    println!("1 + 2 = {}", result1);

    let result2 = calc::mul(3, 4);
    println!("3 * 4 = {}", result2);

    let result3 = calc::div(4.0, 2.0);
    println!("4 / 2 = {}", result3);

    warn!("Something is not right!");
    info!("Shutting down!");
}
