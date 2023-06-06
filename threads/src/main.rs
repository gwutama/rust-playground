use num_cpus;
use std::thread;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");

    // a detached thread
    let detached_thread = thread::spawn(|| {
        println!("I am a detached thread");
    });

    let output = detached_thread.join(); // wait for the thread to finish

    match output {
        Ok(_) => println!("Thread finished successfully"),
        Err(_) => println!("Thread panicked"),
    }

    // spawn multiple threads and wait for them to finish
    let n_workers = num_cpus::get();
    println!("Number of cpus: {}", n_workers);
    let pool = ThreadPool::new(n_workers);

    // synchronize data across threads with a mutex
    let a_number: i32 = 2;
    let shared_data = Arc::new(Mutex::new(a_number));

    for i in 0..n_workers {
        let mutex = shared_data.clone(); // clone the mutex for each thread

        pool.execute(move || { // move the mutex and i into the thread
            println!("Hello from thread {}", i);
            let mut a_number_mutex = mutex.lock().unwrap();
            *a_number_mutex *= 2;

            // simulate a panic
            // if i % 2 == 0 {
            //     panic!("Thread {} panic!", i);
            // }
        });
    }

    pool.join();
    println!("Panic count: {}", pool.panic_count());
    println!("Final value of a_number: {}", *shared_data.lock().unwrap());
}
