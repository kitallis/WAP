extern crate sysfs_gpio;

extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

struct Arguments {
    pin: u64,
    duration_ms: u64,
    period_ms: u64,
}

// Export a GPIO for use. This will not fail if already exported
fn pour(pin_num: u64, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
    let pin = Pin::new(pin_num);

    pin.with_exported(|| {
        pin.set_direction(Direction::Out)?;

        // set a duration for how long the whole drip should run
        // do it in chunks of periods
        // make sure period is smaller than duration, or their division is non-zero
        // write out to a log file the state and time at which it ran
        let iterations = duration_ms / period_ms;
        for _ in 0..iterations {
            pin.set_value(0)?;
            sleep(Duration::from_millis(period_ms));
            pin.set_value(1)?;
            sleep(Duration::from_millis(period_ms));
        }

        pin.set_value(0)?;
        Ok(())
    })
}

fn print_usage() {
    println!("Usage: ./wap <pin> <duration_ms> <period_ms>");
}

fn get_args() -> Option<Arguments> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return None;
    }

    let pin = match args[1].parse::<u64>() {
        Ok(pin) => pin,
        Err(_) => return None,
    };

    let duration_ms = match args[2].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return None,
    };

    let period_ms = match args[3].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return None,
    };

    Some(Arguments {
        pin: pin,
        duration_ms: duration_ms,
        period_ms: period_ms,
    })
}

fn main() {
    match get_args() {
        None => print_usage(),
        Some(args) => match pour(args.pin, args.duration_ms, args.period_ms) {
            Ok(()) => println!("Success!"),
            Err(err) => println!("error, dry-ass plants: {}", err),
        },
    }
}
