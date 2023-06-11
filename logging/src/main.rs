// use log::{debug, error, info, trace, warn};

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::init_from_env(env);

    println!("Hello, world!");
    log::info!("this is an info!");
    log::trace!("this is a trace!");
    log::error!("this is an error!");
    log::debug!("this is a debug!");
}
