use log::{error, info};
use log4rs;

pub mod args;
pub mod drip;

fn init_logger() {
    log4rs::init_file("log.conf.yml", Default::default()).unwrap();
}

fn print_usage() {
    println!("Usage: ./wap <pin> <duration_ms> <period_ms>");
}

fn main() {
    init_logger();

    info!("Starting this sessions' drip");

    match args::get() {
        Some(args) => match drip::it(args.pin, args.duration_ms, args.period_ms) {
            Ok(()) => info!("Finished this sessions' drip"),
            Err(err) => error!("Error, dry-ass plants: {}", err),
        },
        None => print_usage(),
    }
}
