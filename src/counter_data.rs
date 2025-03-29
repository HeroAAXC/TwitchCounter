use std::{fmt::Display, str::Split};

use tokio::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct CounterData {
    pub secs: u64,
    pub from: Instant,
}

impl Display for CounterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.from.elapsed().as_secs();
        let elapsed = if elapsed > self.secs {
            0
        } else {
            self.secs - elapsed
        };
        let minutes = elapsed / 60;
        let secs = elapsed % 60;
        write!(f, "{}:{}", minutes, secs)
    }
}

impl CounterData {
    pub fn new(secs: u64) -> Self {
        Self {
            secs,
            from: Instant::now(),
        }
    }
}



pub fn parse_to_secs(time: &str) -> Option<u64> {
    let mut s: Split<'_, &str> = time.split(":");
    let minutes: u64 = match s.next()?.parse() {
        Ok(s) => s,
        Err(_) => return None,
    };

    let secs = match s.next() {
        Some(s) => match s.parse() {
            Ok(s) => s,
            Err(_) => return None,
        },
        None => 0u64,
    };
    Some(minutes * 60u64 + secs)
}
