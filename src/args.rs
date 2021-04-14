use std::env;

pub struct Arguments {
    pub pin: u64,
    pub duration_ms: u64,
    pub period_ms: u64,
}

pub fn get() -> Option<Arguments> {
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
