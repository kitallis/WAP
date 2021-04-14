use std::env;

pub struct Arguments {
    pub pin: u64,
    pub duration_ms: u64,
    pub period_ms: u64,
}

pub enum InputError {
    ParseError,
    NotEnoughArguments,
    PeriodShouldBeSmallerThanDuration,
}

pub fn get() -> Result<Arguments, InputError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(InputError::NotEnoughArguments);
    }

    let pin = match args[1].parse::<u64>() {
        Ok(pin) => pin,
        Err(_) => return Err(InputError::ParseError),
    };

    let duration_ms = match args[2].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return Err(InputError::ParseError),
    };

    let period_ms = match args[3].parse::<u64>() {
        Ok(ms) => ms,
        Err(_) => return Err(InputError::ParseError),
    };

    if duration_ms <= period_ms {
        return Err(InputError::PeriodShouldBeSmallerThanDuration);
    }

    Ok(Arguments {
        pin: pin,
        duration_ms: duration_ms,
        period_ms: period_ms,
    })
}
