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

    match args::get() {
        Ok(args) => match drip::it(args.pin, args.duration_ms, args.period_ms) {
            Ok(()) => info!("Finished this sessions' drip"),
            Err(err) => error!("Error, dry-ass plants: {}", err),
        },

        Err(args::InputError::ParseError) => {
            println!("Could not parse arguments!");
            print_usage()
        }

        Err(args::InputError::NotEnoughArguments) => {
            println!("Insufficient arguments!");
            print_usage()
        }

        Err(args::InputError::PeriodShouldBeSmallerThanDuration) => {
            println!("duration_ms should be bigger than period_ms.");
            print_usage()
        }
    }
}
