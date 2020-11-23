extern crate sysfs_gpio;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

struct Arguments {
  pin: u64,
  duration_ms: u64,
  period_ms: u64,
}

// Export a GPIO for use. This will not fail if already exported
fn pour(pin: u64, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
  let actionPin = Pin::new(pin);

  actionPin.with_exported(|| {
    actionPin.set_direction(Direction::Low)?;
    let iterations = duration_ms / period_ms / 2;
    for _ in 0..iterations {
      actionPin.set_value(0)?;
      sleep(Duration::from_millis(period_ms));
      actionPin.set_value(1)?;
      sleep(Duration::from_millis(period_ms));
    }

    actionPin.set_value(0)?;
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
