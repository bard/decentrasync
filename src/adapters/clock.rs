use std::{
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

use crate::app::Clock;

pub struct SystemClock {}

impl SystemClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub struct FakeClock {
    clock: Arc<RwLock<SystemTime>>,
}

impl FakeClock {
    pub fn new() -> Self {
        Self {
            clock: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
        }
    }

    pub fn advance(&self, duration: Duration) {
        let mut clock = self.clock.write().unwrap();
        *clock += duration;
    }
}

impl Clock for FakeClock {
    fn now(&self) -> SystemTime {
        *self.clock.read().unwrap()
    }
}
