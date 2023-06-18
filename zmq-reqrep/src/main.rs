use std::env;
use std::thread;
use std::time::Duration;

const BIND: &str = "ipc:///tmp/hello";
// const BIND: &str = "tcp://127.0.0.1:5555";

fn run_server() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind(BIND).is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}

fn run_client() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect(BIND).is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    if args.get(1).unwrap() == "server" {
        println!("Server mode");
        run_server();
    } else {
        println!("Client mode");
        run_client();
    }
}
