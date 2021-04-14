extern crate sysfs_gpio;

use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn init_ctrlc_handler(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        info!("Shutting down drip...");
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}

pub fn it(pin_num: u64, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
    info!("Starting this sessions' drip");

    let pin = Pin::new(pin_num);
    let running = Arc::new(AtomicBool::new(true));
    init_ctrlc_handler(running.clone());

    pin.with_exported(|| {
        pin.set_direction(Direction::Out)?;

        let batches = duration_ms / period_ms;

        info!(
            "Drip for a total of {} seconds in {} batches of {} seconds each",
            duration_ms / 1000,
            batches,
            period_ms / 1000
        );

        // run in batches to avoid over-heating the solenoid
        for _ in 0..batches {
            while running.load(Ordering::SeqCst) {
                pin.set_value(0)?;
                info!("Drip is ON");
                sleep(Duration::from_millis(period_ms));

                pin.set_value(1)?;
                info!("Drip is OFF");
                sleep(Duration::from_millis(period_ms));
            }
        }

        pin.set_value(1)?;
        Ok(())
    })
}
